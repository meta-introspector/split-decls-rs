macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnexpectedParameterName {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unexpected_parameter_name)] pub (crate) struct UnexpectedParameterName { # [primary_span] # [label] pub span : Span , pub prefix_nops : Symbol , pub entry_nops : Symbol , }
    };
}

UnexpectedParameterName!()