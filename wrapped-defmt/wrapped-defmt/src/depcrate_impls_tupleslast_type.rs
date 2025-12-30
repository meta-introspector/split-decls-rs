// Generated macro for last_type (macro)
macro_rules! Depcrate_impls_tupleslast_type {
() => {
// Module: crate::impls::tuples
// Provides: {"last_type"}
// Dependencies: {}
macro_rules ! last_type { ($ a : ident ,) => { $ a } ; ($ a : ident , $ ($ rest_a : ident ,) +) => { last_type ! ($ ($ rest_a ,) +) } ; }
};
}
