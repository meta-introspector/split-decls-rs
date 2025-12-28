macro_rules! BundleNeedsStatic {
    () => {
        # [derive (Diagnostic)] # [diag (attr_parsing_bundle_needs_static)] pub (crate) struct BundleNeedsStatic { # [primary_span] pub span : Span , }
    };
}

BundleNeedsStatic!();