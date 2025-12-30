// Generated macro for table_in_quote (function)
macro_rules! Depcrate_note_teststable_in_quote {
() => {
// Module: crate::note::tests
// Provides: {"table_in_quote"}
// Dependencies: {}
# [test] fn table_in_quote () { let text = "> A table.\n\n| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |" ; let processed = rewrite (text) ; assert_eq ! (render_markdown (& processed) , "<blockquote>\n<p>A table.</p>\n</blockquote>\n<table><thead><tr><th>Header 1</th><th>Header 2</th></tr></thead><tbody>\n<tr><td>Text 123</td><td>More 456</td></tr>\n</tbody></table>\n" , "It renders blockquotes with nested tables as expected.") ; }
};
}
