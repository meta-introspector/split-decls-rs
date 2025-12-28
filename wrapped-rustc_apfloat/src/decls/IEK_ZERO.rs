macro_rules! deps {
    () => {
        ExpInt!();
    };
}

macro_rules! IEK_ZERO {
    () => {
        deps!();
        pub const IEK_ZERO : ExpInt = ExpInt :: min_value () + 1 ;
    };
}

IEK_ZERO!()