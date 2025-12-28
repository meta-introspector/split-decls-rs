macro_rules! deps {
    () => {
        REG_SAM_FLAGS!();
    };
}

macro_rules! KEY_READ {
    () => {
        deps!();
        pub const KEY_READ : REG_SAM_FLAGS = 131097u32 ;
    };
}

KEY_READ!();