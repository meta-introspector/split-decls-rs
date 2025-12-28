macro_rules! MH_NO_HEAP_EXECUTION {
    () => {
        # [doc = " When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes."] pub const MH_NO_HEAP_EXECUTION : u32 = 0x100_0000 ;
    };
}

MH_NO_HEAP_EXECUTION!();