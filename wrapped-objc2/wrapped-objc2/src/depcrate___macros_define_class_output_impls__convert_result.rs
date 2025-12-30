// Generated macro for __convert_result (macro)
macro_rules! Depcrate___macros_define_class_output_impls__convert_result {
() => {
// Module: crate::__macros::define_class::output_impls
// Provides: {"__convert_result"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __convert_result { ($ body : block) => { $ body } ; ($ body : block ; $ ret : ty) => { let __objc2_result = $ body ; # [allow (unreachable_code)] <$ ret as $ crate :: __macros :: ConvertReturn < () >>:: convert_defined_return (__objc2_result) } ; }
};
}
