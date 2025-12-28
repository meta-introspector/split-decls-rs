macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! BinaryOutputToTty {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_binary_output_to_tty)] pub struct BinaryOutputToTty { pub shorthand : & 'static str , }
    };
}

BinaryOutputToTty!();