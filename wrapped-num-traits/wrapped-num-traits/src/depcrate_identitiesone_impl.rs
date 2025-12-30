// Generated macro for one_impl (macro)
macro_rules! Depcrate_identitiesone_impl {
() => {
// Module: crate::identities
// Provides: {"one_impl"}
// Dependencies: {}
macro_rules ! one_impl { ($ t : ty , $ v : expr) => { impl One for $ t { # [inline] fn one () -> $ t { $ v } # [inline] fn is_one (& self) -> bool { * self == $ v } } impl ConstOne for $ t { const ONE : Self = $ v ; } } ; }
};
}
