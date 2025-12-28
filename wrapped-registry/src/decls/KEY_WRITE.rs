macro_rules! deps {
    () => {
        REG_SAM_FLAGS!();
    };
}

macro_rules! KEY_WRITE {
    () => {
        deps!();
        pub const KEY_WRITE : REG_SAM_FLAGS = 131078u32 ;
    };
}

KEY_WRITE!();