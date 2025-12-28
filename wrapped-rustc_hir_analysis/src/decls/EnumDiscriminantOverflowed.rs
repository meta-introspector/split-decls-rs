macro_rules! EnumDiscriminantOverflowed {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_enum_discriminant_overflowed , code = E0370)] # [note] pub (crate) struct EnumDiscriminantOverflowed { # [primary_span] # [label] pub span : Span , pub discr : String , pub item_name : Ident , pub wrapped_discr : String , }
    };
}

EnumDiscriminantOverflowed!()