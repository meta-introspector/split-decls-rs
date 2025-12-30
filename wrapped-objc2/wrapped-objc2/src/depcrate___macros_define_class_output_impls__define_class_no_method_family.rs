// Generated macro for __define_class_no_method_family (macro)
macro_rules! Depcrate___macros_define_class_output_impls__define_class_no_method_family {
() => {
// Module: crate::__macros::define_class::output_impls
// Provides: {"__define_class_no_method_family"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __define_class_no_method_family { () => { } ; ($ ($ t : tt) +) => { $ crate :: __macros :: compile_error ! ("`#[unsafe(method_family = ...)]` is not yet supported in `define_class!` together with `#[unsafe(method(...))]`") } ; }
};
}
