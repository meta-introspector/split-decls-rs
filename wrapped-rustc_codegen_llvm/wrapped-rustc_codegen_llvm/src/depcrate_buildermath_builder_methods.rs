// Generated macro for math_builder_methods (macro)
macro_rules! Depcrate_buildermath_builder_methods {
() => {
// Module: crate::builder
// Provides: {"math_builder_methods"}
// Dependencies: {}
macro_rules ! math_builder_methods { ($ ($ name : ident ($ ($ arg : ident) ,*) => $ llvm_capi : ident) ,+ $ (,) ?) => { $ (fn $ name (& mut self , $ ($ arg : &'ll Value) ,*) -> &'ll Value { unsafe { llvm ::$ llvm_capi (self . llbuilder , $ ($ arg ,) * UNNAMED) } }) + } }
};
}
