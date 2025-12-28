macro_rules! deps {
    () => {
        ExpInt!();
    };
}

macro_rules! IEK_NAN {
    () => {
        deps!();
        pub const IEK_NAN : ExpInt = ExpInt :: min_value () ;
    };
}

IEK_NAN!()