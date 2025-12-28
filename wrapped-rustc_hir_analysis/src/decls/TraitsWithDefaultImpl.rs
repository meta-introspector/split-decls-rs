macro_rules! TraitsWithDefaultImpl {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_traits_with_default_impl , code = E0321)] # [note] pub (crate) struct TraitsWithDefaultImpl < 'a > { # [primary_span] pub span : Span , pub traits : String , pub problematic_kind : & 'a str , pub self_ty : Ty < 'a > , }
    };
}

TraitsWithDefaultImpl!();