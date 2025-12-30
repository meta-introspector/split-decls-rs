// Generated macro for __bind (function)
macro_rules! Depcrate__bind {
() => {
// Module: crate
// Provides: {"__bind"}
// Dependencies: {}
# [doc (hidden)] # [proc_macro] pub fn __bind (input : TokenStream) -> TokenStream { try_bind (input) . unwrap_or_else (| msg | parse_ts (& format ! ("compile_error!({msg:?})"))) }
};
}
