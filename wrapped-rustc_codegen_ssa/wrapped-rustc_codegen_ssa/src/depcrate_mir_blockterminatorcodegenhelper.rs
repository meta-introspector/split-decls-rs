// Generated macro for TerminatorCodegenHelper (struct)
macro_rules! Depcrate_mir_blockTerminatorCodegenHelper {
() => {
// Module: crate::mir::block
// Provides: {"TerminatorCodegenHelper"}
// Dependencies: {}
# [doc = " Used by `FunctionCx::codegen_terminator` for emitting common patterns"] # [doc = " e.g., creating a basic block, calling a function, etc."] struct TerminatorCodegenHelper < 'tcx > { bb : mir :: BasicBlock , terminator : & 'tcx mir :: Terminator < 'tcx > , }
};
}
