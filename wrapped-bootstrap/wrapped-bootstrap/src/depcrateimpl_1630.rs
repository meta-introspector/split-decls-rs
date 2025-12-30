// Generated macro for impl_1630 (impl)
macro_rules! Depcrateimpl_1630 {
() => {
// Module: crate
// Provides: {"impl_1630"}
// Dependencies: {}
impl CodegenBackendKind { # [doc = " Name of the codegen backend, as identified in the `compiler` directory"] # [doc = " (`rustc_codegen_<name>`)."] pub fn name (& self) -> & str { match self { CodegenBackendKind :: Llvm => "llvm" , CodegenBackendKind :: Cranelift => "cranelift" , CodegenBackendKind :: Gcc => "gcc" , CodegenBackendKind :: Custom (name) => name , } } # [doc = " Name of the codegen backend's crate, e.g. `rustc_codegen_cranelift`."] pub fn crate_name (& self) -> String { format ! ("rustc_codegen_{}" , self . name ()) } pub fn is_llvm (& self) -> bool { matches ! (self , Self :: Llvm) } pub fn is_cranelift (& self) -> bool { matches ! (self , Self :: Cranelift) } pub fn is_gcc (& self) -> bool { matches ! (self , Self :: Gcc) } }
};
}
