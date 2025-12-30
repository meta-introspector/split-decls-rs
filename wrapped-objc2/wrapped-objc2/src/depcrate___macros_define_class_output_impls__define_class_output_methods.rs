// Generated macro for __define_class_output_methods (macro)
macro_rules! Depcrate___macros_define_class_output_impls__define_class_output_methods {
() => {
// Module: crate::__macros::define_class::output_impls
// Provides: {"__define_class_output_methods"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_output_methods { { } => { } ; { $ (# [$ ($ m : tt) *]) * unsafe fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ body : block $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __define_class_method_out) (unsafe) ($ name) ($ ($ ret) ?) ($ body) } $ crate :: __define_class_output_methods ! { $ ($ rest) * } } ; { $ (# [$ ($ m : tt) *]) * fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ body : block $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __define_class_method_out) () ($ name) ($ ($ ret) ?) ($ body) } $ crate :: __define_class_output_methods ! { $ ($ rest) * } } ; }
};
}
