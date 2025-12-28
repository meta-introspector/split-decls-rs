macro_rules! AbiX86Interrupt {
    () => {
        # [derive (Diagnostic)] # [diag (ast_passes_abi_x86_interrupt)] # [note] pub (crate) struct AbiX86Interrupt { # [primary_span] pub spans : Vec < Span > , pub param_count : usize , }
    };
}

AbiX86Interrupt!();