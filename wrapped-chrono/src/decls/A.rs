macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! A {
    () => {
        deps!();
        pub (super) const A : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_SATURDAY) ;
    };
}

A!();