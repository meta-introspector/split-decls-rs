// Generated macro for macro_31 (macro)
macro_rules! Depcrate_async_await_join_modmacro_31 {
() => {
// Module: crate::async_await::join_mod
// Provides: {"macro_31"}
// Dependencies: {}
document_join_macro ! { # [macro_export] macro_rules ! join { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: join_internal ! { $ ($ tokens) * } } } } # [macro_export] macro_rules ! try_join { ($ ($ tokens : tt) *) => { { use $ crate :: __private as __futures_crate ; $ crate :: try_join_internal ! { $ ($ tokens) * } } } } }
};
}
