macro_rules! deps {
    () => {
        UNWIND_HISTORY_TABLE!();
        IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY!();
    };
}

macro_rules! macro_88 {
    () => {
        deps!();
        # [cfg (target_arch = "aarch64")] windows_link :: link ! ("kernel32.dll" "system" fn RtlLookupFunctionEntry (controlpc : usize , imagebase : * mut usize , historytable : * mut UNWIND_HISTORY_TABLE) -> * mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) ;
    };
}

macro_88!();