macro_rules! MH_NO_REEXPORTED_DYLIBS {
    () => {
        # [doc = " When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported"] pub const MH_NO_REEXPORTED_DYLIBS : u32 = 0x10_0000 ;
    };
}

MH_NO_REEXPORTED_DYLIBS!();