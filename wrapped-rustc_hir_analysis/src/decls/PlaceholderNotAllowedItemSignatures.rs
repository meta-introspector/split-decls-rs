macro_rules! PlaceholderNotAllowedItemSignatures {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_placeholder_not_allowed_item_signatures , code = E0121)] pub (crate) struct PlaceholderNotAllowedItemSignatures { # [primary_span] # [label] pub spans : Vec < Span > , pub kind : String , }
    };
}

PlaceholderNotAllowedItemSignatures!();