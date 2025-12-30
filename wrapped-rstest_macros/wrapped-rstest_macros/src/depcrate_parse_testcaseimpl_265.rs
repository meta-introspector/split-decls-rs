// Generated macro for impl_265 (impl)
macro_rules! Depcrate_parse_testcaseimpl_265 {
() => {
// Module: crate::parse::testcase
// Provides: {"impl_265"}
// Dependencies: {}
impl Parse for TestCase { fn parse (input : ParseStream) -> Result < Self > { let attrs = Attribute :: parse_outer (input) ? ; let case : Ident = input . parse () ? ; if case == "case" { let mut description = None ; if input . peek (Token ! [::]) { let _ = input . parse :: < Token ! [::] > () ; description = Some (input . parse () ?) ; } let content ; let _ = syn :: parenthesized ! (content in input) ; let args = Punctuated :: < Expr , Token ! [,] > :: parse_terminated (& content) ? . into_iter () . collect () ; Ok (TestCase { args , attrs , description , }) } else { Err (Error :: new (case . span () , "expected a test case")) } } }
};
}
