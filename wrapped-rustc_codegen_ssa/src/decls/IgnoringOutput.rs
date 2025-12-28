macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! IgnoringOutput {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_ignoring_output)] pub struct IgnoringOutput { pub extension : & 'static str , }
    };
}

IgnoringOutput!()