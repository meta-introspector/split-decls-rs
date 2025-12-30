// Generated macro for impl_float_const_method (macro)
macro_rules! Depcrateimpl_float_const_method {
() => {
// Module: crate
// Provides: {"impl_float_const_method"}
// Dependencies: {}
macro_rules ! impl_float_const_method { ($ wrapper : expr , $ method : ident) => { # [allow (non_snake_case)] # [allow (clippy :: redundant_closure_call)] fn $ method () -> Self { $ wrapper (T ::$ method ()) } } ; }
};
}
