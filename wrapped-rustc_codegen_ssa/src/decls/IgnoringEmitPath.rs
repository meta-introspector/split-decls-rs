macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! IgnoringEmitPath {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_ignoring_emit_path)] pub struct IgnoringEmitPath { pub extension : & 'static str , }
    };
}

IgnoringEmitPath!();