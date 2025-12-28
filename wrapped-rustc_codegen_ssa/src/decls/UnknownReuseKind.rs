macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnknownReuseKind {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unknown_reuse_kind)] pub (crate) struct UnknownReuseKind { # [primary_span] pub span : Span , pub kind : Symbol , }
    };
}

UnknownReuseKind!();