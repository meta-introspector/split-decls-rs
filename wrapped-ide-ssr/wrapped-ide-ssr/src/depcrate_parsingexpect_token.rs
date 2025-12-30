// Generated macro for expect_token (function)
macro_rules! Depcrate_parsingexpect_token {
() => {
// Module: crate::parsing
// Provides: {"expect_token"}
// Dependencies: {}
fn expect_token (tokens : & mut std :: vec :: IntoIter < Token > , expected : & str) -> Result < () , SsrError > { if let Some (t) = tokens . next () { if t . text == expected { return Ok (()) ; } bail ! ("Expected {} found {}" , expected , t . text) ; } bail ! ("Expected {} found end of stream" , expected) ; }
};
}
