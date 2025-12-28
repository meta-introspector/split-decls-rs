macro_rules! TransparentEnumVariant {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_transparent_enum_variant , code = E0731)] pub (crate) struct TransparentEnumVariant { # [primary_span] # [label] pub span : Span , # [label (hir_analysis_multi_label)] pub spans : Vec < Span > , # [label (hir_analysis_many_label)] pub many : Option < Span > , pub number : usize , pub path : String , }
    };
}

TransparentEnumVariant!()