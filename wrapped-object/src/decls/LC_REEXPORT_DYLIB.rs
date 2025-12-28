macro_rules! LC_REEXPORT_DYLIB {
    () => {
        # [doc = " load and re-export dylib"] pub const LC_REEXPORT_DYLIB : u32 = 0x1f | LC_REQ_DYLD ;
    };
}

LC_REEXPORT_DYLIB!();