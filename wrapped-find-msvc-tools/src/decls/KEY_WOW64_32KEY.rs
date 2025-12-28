macro_rules! deps {
    () => {
        REG_SAM_FLAGS!();
    };
}

macro_rules! KEY_WOW64_32KEY {
    () => {
        deps!();
        pub const KEY_WOW64_32KEY : REG_SAM_FLAGS = 512u32 ;
    };
}

KEY_WOW64_32KEY!();