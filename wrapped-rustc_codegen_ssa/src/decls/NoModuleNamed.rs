macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! NoModuleNamed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_no_module_named)] pub (crate) struct NoModuleNamed < 'a > { # [primary_span] pub span : Span , pub user_path : & 'a str , pub cgu_name : Symbol , pub cgu_names : String , }
    };
}

NoModuleNamed!();