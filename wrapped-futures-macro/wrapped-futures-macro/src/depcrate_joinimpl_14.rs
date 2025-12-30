// Generated macro for impl_14 (impl)
macro_rules! Depcrate_joinimpl_14 {
() => {
// Module: crate::join
// Provides: {"impl_14"}
// Dependencies: {}
impl Parse for Join { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut join = Self :: default () ; while ! input . is_empty () { join . fut_exprs . push (input . parse :: < Expr > () ?) ; if ! input . is_empty () { input . parse :: < Token ! [,] > () ? ; } } Ok (join) } }
};
}
