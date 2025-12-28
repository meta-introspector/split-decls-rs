macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! StrippingDebugInfoFailed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_stripping_debug_info_failed)] # [note] pub (crate) struct StrippingDebugInfoFailed < 'a > { pub util : & 'a str , pub status : ExitStatus , pub output : String , }
    };
}

StrippingDebugInfoFailed!();