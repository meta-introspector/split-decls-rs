macro_rules! TerminatorCodegenHelper {
    () => {
        # [doc = " Used by `FunctionCx::codegen_terminator` for emitting common patterns"] # [doc = " e.g., creating a basic block, calling a function, etc."] struct TerminatorCodegenHelper < 'tcx > { bb : mir :: BasicBlock , terminator : & 'tcx mir :: Terminator < 'tcx > , }
    };
}

TerminatorCodegenHelper!()