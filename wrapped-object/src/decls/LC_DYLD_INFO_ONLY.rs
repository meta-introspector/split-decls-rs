macro_rules! LC_DYLD_INFO_ONLY {
    () => {
        # [doc = " compressed dyld information only"] pub const LC_DYLD_INFO_ONLY : u32 = 0x22 | LC_REQ_DYLD ;
    };
}

LC_DYLD_INFO_ONLY!();