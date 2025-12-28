macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! MalformedCguName {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_malformed_cgu_name)] pub (crate) struct MalformedCguName { # [primary_span] pub span : Span , pub user_path : String , pub crate_name : String , }
    };
}

MalformedCguName!()