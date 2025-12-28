macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MultipleInstructionSet {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_multiple_instruction_set , code = E0779)] pub (crate) struct MultipleInstructionSet { # [primary_span] pub span : Span , }
    };
}

MultipleInstructionSet!();