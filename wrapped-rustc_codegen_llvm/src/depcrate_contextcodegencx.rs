// Generated macro for CodegenCx (type)
macro_rules! Depcrate_contextCodegenCx {
() => {
// Module: crate::context
// Provides: {"CodegenCx"}
// Dependencies: {}
# [doc = " There is one `CodegenCx` per codegen unit. Each one has its own LLVM"] # [doc = " `llvm::Context` so that several codegen units may be processed in parallel."] # [doc = " All other LLVM data structures in the `CodegenCx` are tied to that `llvm::Context`."] pub (crate) type CodegenCx < 'll , 'tcx > = GenericCx < 'll , FullCx < 'll , 'tcx > > ;
};
}
