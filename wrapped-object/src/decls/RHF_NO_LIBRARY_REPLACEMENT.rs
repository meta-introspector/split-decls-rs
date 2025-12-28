macro_rules! RHF_NO_LIBRARY_REPLACEMENT {
    () => {
        # [doc = " Ignore LD_LIBRARY_PATH"] pub const RHF_NO_LIBRARY_REPLACEMENT : u32 = 1 << 2 ;
    };
}

RHF_NO_LIBRARY_REPLACEMENT!()