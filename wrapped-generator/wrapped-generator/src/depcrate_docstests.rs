// Generated macro for tests (module)
macro_rules! Depcrate_docstests {
() => {
// Module: crate::docs
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: collections :: HashMap ; use pest_meta :: parser ; use pest_meta :: parser :: Rule ; # [test] fn test_doc_comment () { let pairs = match parser :: parse (Rule :: grammar_rules , include_str ! ("../tests/test.pest")) { Ok (pairs) => pairs , Err (_) => panic ! ("error parsing tests/test.pest") , } ; let doc_comment = super :: consume (pairs) ; let mut expected = HashMap :: new () ; expected . insert ("foo" . to_owned () , "Matches foo str, e.g.: `foo`" . to_owned ()) ; expected . insert ("bar" . to_owned () , "Matches bar str\n\n  Indent 2, e.g: `bar` or `foobar`" . to_owned () ,) ; expected . insert ("dar" . to_owned () , "Matches dar\n\nMatch dar description\n" . to_owned () ,) ; assert_eq ! (expected , doc_comment . line_docs) ; assert_eq ! ("A parser for JSON file.\nAnd this is a example for JSON parser.\n\n    indent-4-space\n" , doc_comment . grammar_doc) ; } # [test] fn test_empty_grammar_doc () { assert ! (parser :: parse (Rule :: grammar_rules , "//!") . is_ok ()) ; assert ! (parser :: parse (Rule :: grammar_rules , "///") . is_ok ()) ; assert ! (parser :: parse (Rule :: grammar_rules , "//") . is_ok ()) ; assert ! (parser :: parse (Rule :: grammar_rules , "/// Line Doc") . is_ok ()) ; assert ! (parser :: parse (Rule :: grammar_rules , "//! Grammar Doc") . is_ok ()) ; assert ! (parser :: parse (Rule :: grammar_rules , "// Comment") . is_ok ()) ; } }
};
}
