// Generated macro for zero_impl (macro)
macro_rules! Depcrate_identitieszero_impl {
() => {
// Module: crate::identities
// Provides: {"zero_impl"}
// Dependencies: {}
macro_rules ! zero_impl { ($ t : ty , $ v : expr) => { impl Zero for $ t { # [inline] fn zero () -> $ t { $ v } # [inline] fn is_zero (& self) -> bool { * self == $ v } } impl ConstZero for $ t { const ZERO : Self = $ v ; } } ; }
};
}
