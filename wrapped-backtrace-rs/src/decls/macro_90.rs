macro_rules! deps {
    () => {
        EXCEPTION_ROUTINE!();
        IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY!();
        KNONVOLATILE_CONTEXT_POINTERS!();
        CONTEXT!();
        RTL_VIRTUAL_UNWIND_HANDLER_TYPE!();
    };
}

macro_rules! macro_90 {
    () => {
        deps!();
        # [cfg (target_arch = "aarch64")] windows_link :: link ! ("kernel32.dll" "system" fn RtlVirtualUnwind (handlertype : RTL_VIRTUAL_UNWIND_HANDLER_TYPE , imagebase : usize , controlpc : usize , functionentry : * const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY , contextrecord : * mut CONTEXT , handlerdata : * mut * mut core :: ffi :: c_void , establisherframe : * mut usize , contextpointers : * mut KNONVOLATILE_CONTEXT_POINTERS) -> EXCEPTION_ROUTINE) ;
    };
}

macro_90!()