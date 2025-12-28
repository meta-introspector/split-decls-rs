macro_rules! MH_DYLIB {
    () => {
        # [doc = " dynamically bound shared library"] pub const MH_DYLIB : u32 = 0x6 ;
    };
}

MH_DYLIB!();