macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! BareInstructionSet {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_bare_instruction_set , code = E0778)] pub (crate) struct BareInstructionSet { # [primary_span] pub span : Span , }
    };
}

BareInstructionSet!()