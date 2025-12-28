macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnableToWriteDebuggerVisualizer {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unable_to_write_debugger_visualizer)] pub (crate) struct UnableToWriteDebuggerVisualizer { pub path : PathBuf , pub error : Error , }
    };
}

UnableToWriteDebuggerVisualizer!()