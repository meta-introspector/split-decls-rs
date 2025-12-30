// Generated macro for __extern_methods_method_id_deprecated (macro)
macro_rules! Depcrate___macros_extern_methods__extern_methods_method_id_deprecated {
() => {
// Module: crate::__macros::extern_methods
// Provides: {"__extern_methods_method_id_deprecated"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_methods_method_id_deprecated { (method ($ ($ sel : tt) *)) => { } ; ($ method_id : ident ($ ($ sel : tt) *)) => { { # [deprecated = $ crate :: __macros :: concat ! ("using #[unsafe(method_id(" , $ crate :: __macros :: stringify ! ($ ($ sel) *) , "))] inside extern_methods! is deprecated.\nUse #[unsafe(method(" , $ crate :: __macros :: stringify ! ($ ($ sel) *) , "))] instead" ,)] # [inline] fn $ method_id () { } $ method_id () ; } } ; }
};
}
