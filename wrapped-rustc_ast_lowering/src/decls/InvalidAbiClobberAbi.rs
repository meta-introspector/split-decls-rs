macro_rules! InvalidAbiClobberAbi {
    () => {
        # [derive (Diagnostic)] # [note] # [diag (ast_lowering_invalid_abi_clobber_abi)] pub (crate) struct InvalidAbiClobberAbi { # [primary_span] pub abi_span : Span , pub supported_abis : String , }
    };
}

InvalidAbiClobberAbi!();