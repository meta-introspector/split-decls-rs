macro_rules! DropImplOnWrongItem {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_drop_impl_on_wrong_item , code = E0120)] pub (crate) struct DropImplOnWrongItem { # [primary_span] # [label] pub span : Span , pub trait_ : Symbol , }
    };
}

DropImplOnWrongItem!();