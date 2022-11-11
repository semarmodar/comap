use comap;

#[test]
fn parsing_header_and_paragraph() {
	let input = "# Header
#this should be paragraph".to_string();
	let expected = "<h1>Header<br><p>this should be paragraph</p>";
	assert_eq!(expected, comap::parser(input));
} 
