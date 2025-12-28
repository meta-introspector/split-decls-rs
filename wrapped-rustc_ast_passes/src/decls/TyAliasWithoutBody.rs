macro_rules! TyAliasWithoutBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_ty_alias_without_body)] pub (crate) struct TyAliasWithoutBody { # [primary_span] pub span : Span , # [suggestion (code = " = <type>;" , applicability = "has-placeholders")] pub replace_span : Span , }
    };
}

TyAliasWithoutBody!()