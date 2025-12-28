macro_rules! LC_RPATH {
    () => {
        # [doc = " runpath additions"] pub const LC_RPATH : u32 = 0x1c | LC_REQ_DYLD ;
    };
}

LC_RPATH!();