macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! FALSE {
    () => {
        deps!();
        pub const FALSE : BOOL = 0i32 ;
    };
}

FALSE!();