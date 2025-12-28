macro_rules! deps {
    () => {
        ExpInt!();
    };
}

macro_rules! IEK_INF {
    () => {
        deps!();
        pub const IEK_INF : ExpInt = ExpInt :: max_value () ;
    };
}

IEK_INF!()