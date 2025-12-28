macro_rules! TransparentNonZeroSizedEnum {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_transparent_non_zero_sized_enum , code = E0690)] pub (crate) struct TransparentNonZeroSizedEnum < 'a > { # [primary_span] # [label] pub span : Span , # [label (hir_analysis_labels)] pub spans : Vec < Span > , pub field_count : usize , pub desc : & 'a str , }
    };
}

TransparentNonZeroSizedEnum!();