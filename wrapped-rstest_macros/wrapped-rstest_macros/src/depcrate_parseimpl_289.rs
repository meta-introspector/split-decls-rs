// Generated macro for impl_289 (impl)
macro_rules! Depcrate_parseimpl_289 {
() => {
// Module: crate::parse
// Provides: {"impl_289"}
// Dependencies: {}
impl Parse for Positional { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Self (Punctuated :: < syn :: Expr , Token ! [,] > :: parse_terminated (input) ? . into_iter () . collect () ,)) } }
};
}
