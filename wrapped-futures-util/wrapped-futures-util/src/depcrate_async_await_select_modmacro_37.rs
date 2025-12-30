// Generated macro for macro_37 (macro)
macro_rules! Depcrate_async_await_select_modmacro_37 {
() => {
// Module: crate::async_await::select_mod
// Provides: {"macro_37"}
// Dependencies: {}
document_select_macro ! { # [cfg (feature = "std")] # [macro_export] macro_rules ! select { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: select_internal ! { $ ($ tokens) * } } } } # [macro_export] macro_rules ! select_biased { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: select_biased_internal ! { $ ($ tokens) * } } } } }
};
}
