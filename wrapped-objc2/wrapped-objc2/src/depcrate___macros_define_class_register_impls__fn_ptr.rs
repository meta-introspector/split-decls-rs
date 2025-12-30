// Generated macro for __fn_ptr (macro)
macro_rules! Depcrate___macros_define_class_register_impls__fn_ptr {
() => {
// Module: crate::__macros::define_class::register_impls
// Provides: {"__fn_ptr"}
// Dependencies: {}
# [doc = " Create function pointer type with inferred parameters."] # [doc (hidden)] # [macro_export] macro_rules ! __fn_ptr { (($ ($ qualifiers : tt) *) ($ ($ output : tt) *) $ (,) ?) => { $ ($ qualifiers) * extern "C-unwind" fn ($ ($ output) *) -> _ } ; (($ ($ qualifiers : tt) *) ($ ($ output : tt) *) _ : $ param_ty : ty $ (, $ ($ rest : tt) *) ?) => { $ crate :: __fn_ptr ! { ($ ($ qualifiers) *) ($ ($ output) * _ ,) $ ($ ($ rest) *) ? } } ; (($ ($ qualifiers : tt) *) ($ ($ output : tt) *) mut $ param : ident : $ param_ty : ty $ (, $ ($ rest : tt) *) ?) => { $ crate :: __fn_ptr ! { ($ ($ qualifiers) *) ($ ($ output) * _ ,) $ ($ ($ rest) *) ? } } ; (($ ($ qualifiers : tt) *) ($ ($ output : tt) *) $ param : ident : $ param_ty : ty $ (, $ ($ rest : tt) *) ?) => { $ crate :: __fn_ptr ! { ($ ($ qualifiers) *) ($ ($ output) * _ ,) $ ($ ($ rest) *) ? } } ; }
};
}
