macro_rules! MH_DYLIB_IN_CACHE {
    () => {
        # [doc = " Only for use on dylibs. When this bit is set, the dylib is part of the dyld"] # [doc = " shared cache, rather than loose in the filesystem."] pub const MH_DYLIB_IN_CACHE : u32 = 0x8000_0000 ;
    };
}

MH_DYLIB_IN_CACHE!();