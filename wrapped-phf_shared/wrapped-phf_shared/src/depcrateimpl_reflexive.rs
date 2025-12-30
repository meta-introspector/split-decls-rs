// Generated macro for impl_reflexive (macro)
macro_rules! Depcrateimpl_reflexive {
() => {
// Module: crate
// Provides: {"impl_reflexive"}
// Dependencies: {}
# [doc = " `impl PhfBorrow<T> for T`"] macro_rules ! impl_reflexive (($ ($ t : ty) ,*) => ($ (impl PhfBorrow <$ t > for $ t { fn borrow (& self) -> &$ t { self } }) *)) ;
};
}
