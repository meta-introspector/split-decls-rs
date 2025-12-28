macro_rules! AddressOfTemporaryTaken {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_address_of_temporary_taken , code = E0745)] pub (crate) struct AddressOfTemporaryTaken { # [primary_span] # [label] pub span : Span , }
    };
}

AddressOfTemporaryTaken!()