// Generated macro for normal_table (function)
macro_rules! Depcrate_note_testsnormal_table {
() => {
// Module: crate::note::tests
// Provides: {"normal_table"}
// Dependencies: {}
# [test] fn normal_table () { let text = "| Header 1 | Header 2 |\n| -------- | -------- |\n| Text 123 | More 456 |" ; let processed = rewrite (text) ; assert_eq ! (processed , "|Header 1|Header 2|\n|--------|--------|\n|Text 123|More 456|" , "It strips some whitespace but otherwise leaves the table intact.") ; }
};
}
