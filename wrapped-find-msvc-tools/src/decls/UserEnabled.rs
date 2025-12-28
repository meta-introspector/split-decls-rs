macro_rules! deps {
    () => {
        MACHINE_ATTRIBUTES!();
    };
}

macro_rules! UserEnabled {
    () => {
        deps!();
        pub const UserEnabled : MACHINE_ATTRIBUTES = 1i32 ;
    };
}

UserEnabled!();