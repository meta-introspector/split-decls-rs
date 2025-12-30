// Generated macro for set_math_builder_methods (macro)
macro_rules! Depcrate_builderset_math_builder_methods {
() => {
// Module: crate::builder
// Provides: {"set_math_builder_methods"}
// Dependencies: {}
macro_rules ! set_math_builder_methods { ($ ($ name : ident ($ ($ arg : ident) ,*) => ($ llvm_capi : ident , $ llvm_set_math : ident)) ,+ $ (,) ?) => { $ (fn $ name (& mut self , $ ($ arg : &'ll Value) ,*) -> &'ll Value { unsafe { let instr = llvm ::$ llvm_capi (self . llbuilder , $ ($ arg ,) * UNNAMED) ; llvm ::$ llvm_set_math (instr) ; instr } }) + } }
};
}
