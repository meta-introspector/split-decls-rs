macro_rules! MH_DYLIB_STUB {
    () => {
        # [doc = " shared library stub for static linking only, no section contents"] pub const MH_DYLIB_STUB : u32 = 0x9 ;
    };
}

MH_DYLIB_STUB!();