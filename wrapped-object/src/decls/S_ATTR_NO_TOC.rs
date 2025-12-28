macro_rules! S_ATTR_NO_TOC {
    () => {
        # [doc = " section contains coalesced symbols that are not to be in a ranlib table of contents"] pub const S_ATTR_NO_TOC : u32 = 0x4000_0000 ;
    };
}

S_ATTR_NO_TOC!()