macro_rules! S_LAZY_DYLIB_SYMBOL_POINTERS {
    () => {
        # [doc = " section with only lazy symbol pointers to lazy loaded dylibs"] pub const S_LAZY_DYLIB_SYMBOL_POINTERS : u32 = 0x10 ;
    };
}

S_LAZY_DYLIB_SYMBOL_POINTERS!();