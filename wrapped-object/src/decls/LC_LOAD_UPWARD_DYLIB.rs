macro_rules! LC_LOAD_UPWARD_DYLIB {
    () => {
        # [doc = " load upward dylib"] pub const LC_LOAD_UPWARD_DYLIB : u32 = 0x23 | LC_REQ_DYLD ;
    };
}

LC_LOAD_UPWARD_DYLIB!()