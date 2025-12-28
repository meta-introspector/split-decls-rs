macro_rules! S_ATTR_STRIP_STATIC_SYMS {
    () => {
        # [doc = " ok to strip static symbols in this section in files with the MH_DYLDLINK flag"] pub const S_ATTR_STRIP_STATIC_SYMS : u32 = 0x2000_0000 ;
    };
}

S_ATTR_STRIP_STATIC_SYMS!()