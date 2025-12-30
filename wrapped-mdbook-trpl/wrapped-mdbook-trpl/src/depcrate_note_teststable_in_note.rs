// Generated macro for table_in_note (function)
macro_rules! Depcrate_note_teststable_in_note {
() => {
// Module: crate::note::tests
// Provides: {"table_in_note"}
// Dependencies: {}
# [test] fn table_in_note () { let text = "> Note: table stuff.\n\n| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |" ; let processed = rewrite (text) ; assert_eq ! (processed , "\n\n<section class=\"note\" aria-role=\"note\">\n\nNote: table stuff.\n\n</section>\n\n|Header 1|Header 2|\n|--------|--------|\n|Text 123|More 456|" , "It adds the note markup but leaves the table untouched, to be rendered as Markdown.") ; }
};
}
