macro_rules! deps {
    () => {
        ADDRESS_MODE!();
    };
}

macro_rules! AddrModeFlat {
    () => {
        deps!();
        pub const AddrModeFlat : ADDRESS_MODE = 3i32 ;
    };
}

AddrModeFlat!();