macro_rules! ValueOfAssociatedStructAlreadySpecified {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_value_of_associated_struct_already_specified , code = E0719)] pub (crate) struct ValueOfAssociatedStructAlreadySpecified { # [primary_span] # [label] pub span : Span , # [label (hir_analysis_previous_bound_label)] pub prev_span : Span , pub item_name : Ident , pub def_path : String , }
    };
}

ValueOfAssociatedStructAlreadySpecified!()