// Generated macro for CodegenBackendKind (enum)
macro_rules! DepcrateCodegenBackendKind {
() => {
// Module: crate
// Provides: {"CodegenBackendKind"}
// Dependencies: {}
# [doc = " Represents a codegen backend."] # [derive (Debug , Clone , PartialEq , Eq , Hash , Default)] pub enum CodegenBackendKind { # [default] Llvm , Cranelift , Gcc , Custom (String) , }
};
}
