// Generated macro for impl_1252 (impl)
macro_rules! Depcrateimpl_1252 {
() => {
// Module: crate
// Provides: {"impl_1252"}
// Dependencies: {}
impl CompiledModule { # [doc = " Call `emit` function with every artifact type currently compiled"] pub fn for_each_output (& self , mut emit : impl FnMut (& Path , OutputType)) { if let Some (path) = self . object . as_deref () { emit (path , OutputType :: Object) ; } if let Some (path) = self . bytecode . as_deref () { emit (path , OutputType :: Bitcode) ; } if let Some (path) = self . llvm_ir . as_deref () { emit (path , OutputType :: LlvmAssembly) ; } if let Some (path) = self . assembly . as_deref () { emit (path , OutputType :: Assembly) ; } } }
};
}
