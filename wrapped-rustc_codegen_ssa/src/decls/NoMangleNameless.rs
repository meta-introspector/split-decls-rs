macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! NoMangleNameless {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_no_mangle_nameless)] pub (crate) struct NoMangleNameless { # [primary_span] pub span : Span , pub definition : String , }
    };
}

NoMangleNameless!();