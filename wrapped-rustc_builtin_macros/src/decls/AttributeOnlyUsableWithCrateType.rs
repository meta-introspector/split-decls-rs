macro_rules! AttributeOnlyUsableWithCrateType {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_proc_macro_attribute_only_usable_with_crate_type)] pub (crate) struct AttributeOnlyUsableWithCrateType < 'a > { # [primary_span] pub span : Span , pub path : & 'a str , }
    };
}

AttributeOnlyUsableWithCrateType!();