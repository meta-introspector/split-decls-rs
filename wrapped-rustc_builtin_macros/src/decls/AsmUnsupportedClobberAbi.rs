macro_rules! AsmUnsupportedClobberAbi {
    () => {
        # [derive (Diagnostic)] # [diag (builtin_macros_asm_unsupported_clobber_abi)] pub (crate) struct AsmUnsupportedClobberAbi { # [primary_span] pub (crate) spans : Vec < Span > , pub (crate) macro_name : & 'static str , }
    };
}

AsmUnsupportedClobberAbi!();