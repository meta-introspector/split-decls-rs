// Generated macro for __define_class_register_out (macro)
macro_rules! Depcrate___macros_define_class_register_impls__define_class_register_out {
() => {
// Module: crate::__macros::define_class::register_impls
// Provides: {"__define_class_register_out"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_register_out { { ($ builder : ident) ($ ($ qualifiers : tt) *) ($ name : ident) ($ method_or_method_id : ident ($ ($ sel : tt) *)) ($ ($ method_family : tt) *) ($ ($ optional : tt) *) ($ ($ attr_method : tt) *) ($ ($ attr_use : tt) *) ($ builder_method : ident) ($ __receiver : expr) ($ __receiver_ty : ty) ($ ($ __params_prefix : tt) *) ($ ($ params_rest : tt) *) } => { $ ($ attr_use) * { $ crate :: __define_class_invalid_selectors ! ($ method_or_method_id ($ ($ sel) *)) ; $ crate :: __define_class_no_optional ! ($ ($ optional) *) ; $ builder .$ builder_method ($ crate :: sel ! ($ ($ sel) *) , Self ::$ name as $ crate :: __fn_ptr ! { ($ ($ qualifiers) *) (_ , _ ,) $ ($ params_rest) * } ,) ; } } ; }
};
}
