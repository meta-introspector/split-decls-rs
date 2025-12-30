// Generated macro for __define_class_method_out (macro)
macro_rules! Depcrate___macros_define_class_output_impls__define_class_method_out {
() => {
// Module: crate::__macros::define_class::output_impls
// Provides: {"__define_class_method_out"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_method_out { { ($ ($ qualifiers : tt) *) ($ name : ident) ($ ($ ret : ty) ?) ($ body : block) ($ ($ m_method : tt) *) ($ ($ method_family : tt) *) ($ ($ optional : tt) *) ($ ($ attr_method : tt) *) ($ ($ attr_use : tt) *) ($ builder_method : ident) ($ receiver : expr) ($ receiver_ty : ty) ($ ($ params_prefix : tt) *) ($ ($ params_rest : tt) *) } => { $ crate :: __define_class_rewrite_params ! { ($ ($ params_rest) *) () () ($ crate :: __define_class_method_out_inner) ($ ($ qualifiers) *) ($ name) ($ ($ ret) ?) ($ body) ($ builder_method) ($ receiver) ($ receiver_ty) ($ ($ params_prefix) *) ($ ($ m_method) *) ($ ($ method_family) *) ($ ($ optional) *) ($ ($ attr_method) *) ($ ($ attr_use) *) } } ; }
};
}
