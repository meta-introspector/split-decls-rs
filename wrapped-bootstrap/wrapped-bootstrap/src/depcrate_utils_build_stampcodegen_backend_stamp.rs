// Generated macro for codegen_backend_stamp (function)
macro_rules! Depcrate_utils_build_stampcodegen_backend_stamp {
() => {
// Module: crate::utils::build_stamp
// Provides: {"codegen_backend_stamp"}
// Dependencies: {}
# [doc = " Cargo's output path for librustc_codegen_llvm in a given stage, compiled by a particular"] # [doc = " compiler for the specified target and backend."] pub fn codegen_backend_stamp (builder : & Builder < '_ > , compiler : Compiler , target : TargetSelection , backend : & CodegenBackendKind ,) -> BuildStamp { BuildStamp :: new (& builder . cargo_out (compiler , Mode :: Codegen , target)) . with_prefix (& format ! ("lib{}" , backend . crate_name ())) }
};
}
