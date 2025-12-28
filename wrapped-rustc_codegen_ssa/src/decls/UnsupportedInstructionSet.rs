macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnsupportedInstructionSet {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unsupported_instruction_set , code = E0779)] pub (crate) struct UnsupportedInstructionSet { # [primary_span] pub span : Span , }
    };
}

UnsupportedInstructionSet!()