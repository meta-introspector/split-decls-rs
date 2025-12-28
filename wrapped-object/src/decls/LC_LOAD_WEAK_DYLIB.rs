macro_rules! LC_LOAD_WEAK_DYLIB {
    () => {
        # [doc = " load a dynamically linked shared library that is allowed to be missing"] # [doc = " (all symbols are weak imported)."] pub const LC_LOAD_WEAK_DYLIB : u32 = 0x18 | LC_REQ_DYLD ;
    };
}

LC_LOAD_WEAK_DYLIB!()