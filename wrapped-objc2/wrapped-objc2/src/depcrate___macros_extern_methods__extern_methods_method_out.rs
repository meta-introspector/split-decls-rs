// Generated macro for __extern_methods_method_out (macro)
macro_rules! Depcrate___macros_extern_methods__extern_methods_method_out {
() => {
// Module: crate::__macros::extern_methods
// Provides: {"__extern_methods_method_out"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __extern_methods_method_out { { ($ ($ function_start : tt) *) ($ method_or_method_id : ident ($ ($ sel : tt) *)) ($ ($ method_family : tt) *) ($ ($ optional : tt) *) ($ ($ attr_method : tt) *) ($ ($ attr_use : tt) *) ($ __builder_method : ident) ($ receiver : expr) ($ __receiver_ty : ty) ($ ($ __params_prefix : tt) *) ($ ($ params_rest : tt) *) } => { $ ($ attr_method) * $ ($ function_start) * { $ crate :: __extern_methods_method_id_deprecated ! ($ method_or_method_id ($ ($ sel) *)) ; $ crate :: __extern_methods_no_optional ! ($ ($ optional) *) ; # [allow (unused_unsafe)] unsafe { $ crate :: __method_msg_send ! { ($ receiver) ($ ($ sel) *) ($ ($ params_rest) *) () () ($ ($ method_family) *) } } } } ; }
};
}
