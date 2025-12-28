macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! InvalidInstructionSet {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_invalid_instruction_set , code = E0779)] pub (crate) struct InvalidInstructionSet { # [primary_span] pub span : Span , }
    };
}

InvalidInstructionSet!()