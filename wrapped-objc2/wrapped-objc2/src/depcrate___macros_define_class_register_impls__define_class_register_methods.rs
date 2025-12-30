// Generated macro for __define_class_register_methods (macro)
macro_rules! Depcrate___macros_define_class_register_impls__define_class_register_methods {
() => {
// Module: crate::__macros::define_class::register_impls
// Provides: {"__define_class_register_methods"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_register_methods { { ($ builder : ident) } => { } ; { ($ builder : ident) $ (# [$ ($ m : tt) *]) * unsafe fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ body : block $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __define_class_register_out) ($ builder) (unsafe) ($ name) } $ crate :: __define_class_register_methods ! { ($ builder) $ ($ rest) * } } ; { ($ builder : ident) $ (# [$ ($ m : tt) *]) * fn $ name : ident ($ ($ params : tt) *) $ (-> $ ret : ty) ? $ body : block $ ($ rest : tt) * } => { $ crate :: __extract_method_attributes ! { ($ (# [$ ($ m) *]) *) ($ crate :: __rewrite_self_param) ($ ($ params) *) ($ crate :: __define_class_register_out) ($ builder) () ($ name) } $ crate :: __define_class_register_methods ! { ($ builder) $ ($ rest) * } } ; { ($ builder : ident) $ _associated_item : item $ ($ rest : tt) * } => { $ crate :: __define_class_register_methods ! { ($ builder) $ ($ rest) * } } }
};
}
