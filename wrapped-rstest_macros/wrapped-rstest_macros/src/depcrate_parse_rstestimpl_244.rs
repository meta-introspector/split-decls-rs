// Generated macro for impl_244 (impl)
macro_rules! Depcrate_parse_rstestimpl_244 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_244"}
// Dependencies: {}
impl Parse for RsTestData { fn parse (input : ParseStream) -> syn :: Result < Self > { if input . peek (Token ! [::]) { Ok (Default :: default ()) } else { Ok (Self { items : parse_vector_trailing_till_double_comma :: < _ , Token ! [,] > (input) ? , }) } } }
};
}
