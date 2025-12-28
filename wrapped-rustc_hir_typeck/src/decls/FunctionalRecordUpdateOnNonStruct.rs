macro_rules! FunctionalRecordUpdateOnNonStruct {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_functional_record_update_on_non_struct , code = E0436)] pub (crate) struct FunctionalRecordUpdateOnNonStruct { # [primary_span] pub span : Span , }
    };
}

FunctionalRecordUpdateOnNonStruct!();