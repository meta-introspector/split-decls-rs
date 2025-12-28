macro_rules! deps {
    () => {
        CREATE_TOOLHELP_SNAPSHOT_FLAGS!();
    };
}

macro_rules! TH32CS_SNAPMODULE {
    () => {
        deps!();
        pub const TH32CS_SNAPMODULE : CREATE_TOOLHELP_SNAPSHOT_FLAGS = 8u32 ;
    };
}

TH32CS_SNAPMODULE!();