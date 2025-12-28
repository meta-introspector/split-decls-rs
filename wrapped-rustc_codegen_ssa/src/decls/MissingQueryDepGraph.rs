macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MissingQueryDepGraph {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_missing_query_depgraph)] pub (crate) struct MissingQueryDepGraph { # [primary_span] pub span : Span , }
    };
}

MissingQueryDepGraph!()