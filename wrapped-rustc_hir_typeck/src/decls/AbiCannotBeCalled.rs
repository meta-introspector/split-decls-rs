macro_rules! AbiCannotBeCalled {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_abi_cannot_be_called)] pub (crate) struct AbiCannotBeCalled { # [primary_span] # [note] pub span : Span , pub abi : ExternAbi , }
    };
}

AbiCannotBeCalled!()