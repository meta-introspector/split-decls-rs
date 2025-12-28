macro_rules! LC_MAIN {
    () => {
        # [doc = " replacement for LC_UNIXTHREAD"] pub const LC_MAIN : u32 = 0x28 | LC_REQ_DYLD ;
    };
}

LC_MAIN!()