// Generated macro for method_impl (macro)
macro_rules! Depcrate_runtime_method_implementationmethod_impl {
() => {
// Module: crate::runtime::method_implementation
// Provides: {"method_impl"}
// Dependencies: {}
macro_rules ! method_impl { ($ ($ t : ident) ,*) => { method_impl_inner ! ((unsafe) "C" ; $ ($ t) ,*) ; method_impl_inner ! ("C" ; $ ($ t) ,*) ; method_impl_inner ! ((unsafe) "C-unwind" ; $ ($ t) ,*) ; method_impl_inner ! ("C-unwind" ; $ ($ t) ,*) ; } ; }
};
}
