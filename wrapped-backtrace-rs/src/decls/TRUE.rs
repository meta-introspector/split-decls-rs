macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! TRUE {
    () => {
        deps!();
        pub const TRUE : BOOL = 1i32 ;
    };
}

TRUE!()