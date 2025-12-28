macro_rules! deps {
    () => {
        IMAGE_RUNTIME_FUNCTION_ENTRY!();
        UNWIND_HISTORY_TABLE!();
    };
}

macro_rules! macro_89 {
    () => {
        deps!();
        # [cfg (any (target_arch = "arm64ec" , target_arch = "x86_64"))] windows_link :: link ! ("kernel32.dll" "system" fn RtlLookupFunctionEntry (controlpc : u64 , imagebase : * mut u64 , historytable : * mut UNWIND_HISTORY_TABLE) -> * mut IMAGE_RUNTIME_FUNCTION_ENTRY) ;
    };
}

macro_89!();