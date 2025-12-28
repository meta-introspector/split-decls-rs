macro_rules! ClobberAbiNotSupported {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_clobber_abi_not_supported)] pub (crate) struct ClobberAbiNotSupported { # [primary_span] pub abi_span : Span , }
    };
}

ClobberAbiNotSupported!()