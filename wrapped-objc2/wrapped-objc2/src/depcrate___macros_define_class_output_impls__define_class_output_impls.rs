// Generated macro for __define_class_output_impls (macro)
macro_rules! Depcrate___macros_define_class_output_impls__define_class_output_impls {
() => {
// Module: crate::__macros::define_class::output_impls
// Provides: {"__define_class_output_impls"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_output_impls { () => { } ; ($ (# [$ m : meta]) * unsafe impl $ protocol : ident for $ for : ty { $ ($ methods : tt) * } $ ($ rest : tt) *) => { $ (# [$ m]) * unsafe impl $ protocol for $ for { } $ (# [$ m]) * impl $ for { $ crate :: __define_class_output_methods ! { $ ($ methods) * } } $ crate :: __define_class_output_impls ! { $ ($ rest) * } } ; ($ (# [$ m : meta]) * impl $ for : ty { $ ($ methods : tt) * } $ ($ rest : tt) *) => { $ (# [$ m]) * impl $ for { $ crate :: __define_class_output_methods ! { $ ($ methods) * } } $ crate :: __define_class_output_impls ! { $ ($ rest) * } } ; }
};
}
