// Generated macro for __extern_protocol_rewrite_methods (macro)
macro_rules! Depcrate___macros_extern_protocol__extern_protocol_rewrite_methods {
() => {
// Module: crate::__macros::extern_protocol
// Provides: {"__extern_protocol_rewrite_methods"}
// Dependencies: {}
# [doc = " tt-munch each protocol method."] # [doc (hidden)] # [macro_export] macro_rules ! __extern_protocol_rewrite_methods { { } => { } ; { $ (# [$ ($ m : tt) *]) * $ v : vis unsafe fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ (where $ ($ where : ty : $ bound : path) ,+ $ (,) ?) ?; $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __extern_protocol_method_out) ($ v unsafe fn $ name ($ ($ params) *) $ (-> $ ret) ?) ($ ($ ($ where : $ bound ,) +) ?) } $ crate :: __extern_protocol_rewrite_methods ! { $ ($ rest) * } } ; { $ (# [$ ($ m : tt) *]) * $ v : vis fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ (where $ ($ where : ty : $ bound : path) ,+ $ (,) ?) ?; $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __extern_protocol_method_out) ($ v fn $ name ($ ($ params) *) $ (-> $ ret) ?) ($ ($ ($ where : $ bound ,) +) ?) } $ crate :: __extern_protocol_rewrite_methods ! { $ ($ rest) * } } ; }
};
}
