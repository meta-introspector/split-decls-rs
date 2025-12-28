macro_rules! LC_LAZY_LOAD_DYLIB {
    () => {
        # [doc = " delay load of dylib until first use"] pub const LC_LAZY_LOAD_DYLIB : u32 = 0x20 ;
    };
}

LC_LAZY_LOAD_DYLIB!();