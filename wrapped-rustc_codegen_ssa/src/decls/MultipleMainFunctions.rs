macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MultipleMainFunctions {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_multiple_main_functions)] # [help] pub (crate) struct MultipleMainFunctions { # [primary_span] pub span : Span , }
    };
}

MultipleMainFunctions!()