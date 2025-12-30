// Generated macro for comment (module)
macro_rules! Depcrate_parse_nom_testscomment {
() => {
// Module: crate::parse::nom::tests
// Provides: {"comment"}
// Dependencies: {}
mod comment { use winnow :: prelude :: * ; use super :: comment ; use crate :: parse :: tests :: util :: { comment as parsed_comment , fully_consumed } ; # [test] fn semicolon () { assert_eq ! (comment . parse_peek (b"; this is a semicolon comment") . unwrap () , fully_consumed (parsed_comment (';' , " this is a semicolon comment")) ,) ; } # [test] fn octothorpe () { assert_eq ! (comment . parse_peek (b"# this is an octothorpe comment") . unwrap () , fully_consumed (parsed_comment ('#' , " this is an octothorpe comment")) ,) ; } # [test] fn multiple_markers () { assert_eq ! (comment . parse_peek (b"###### this is an octothorpe comment") . unwrap () , fully_consumed (parsed_comment ('#' , "##### this is an octothorpe comment")) ,) ; } }
};
}
