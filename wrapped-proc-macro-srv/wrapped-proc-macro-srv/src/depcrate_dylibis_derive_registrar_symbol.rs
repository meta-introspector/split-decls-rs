// Generated macro for is_derive_registrar_symbol (function)
macro_rules! Depcrate_dylibis_derive_registrar_symbol {
() => {
// Module: crate::dylib
// Provides: {"is_derive_registrar_symbol"}
// Dependencies: {}
fn is_derive_registrar_symbol (symbol : & str) -> bool { const NEW_REGISTRAR_SYMBOL : & str = "_rustc_proc_macro_decls_" ; symbol . contains (NEW_REGISTRAR_SYMBOL) }
};
}
