// Generated macro for __extern_protocol_method_id_deprecated (macro)
macro_rules! Depcrate___macros_extern_protocol__extern_protocol_method_id_deprecated {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__extern_protocol_method_id_deprecated"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_protocol_method_id_deprecated { (method ($ ($ sel : tt) *)) => { } ; (method_id ($ ($ sel : tt) *)) => { { # [deprecated = $ crate :: __macros :: concat ! ("using #[unsafe(method_id(" , $ crate :: __macros :: stringify ! ($ ($ sel) *) , "))] inside extern_protocol! is deprecated.\nUse #[unsafe(method(" , $ crate :: __macros :: stringify ! ($ ($ sel) *) , "))] instead" ,)] # [inline] fn method_id () { } method_id () ; } } ; }
};
}
