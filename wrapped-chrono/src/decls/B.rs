macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! B {
    () => {
        deps!();
        pub (super) const B : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_FRIDAY) ;
    };
}

B!()