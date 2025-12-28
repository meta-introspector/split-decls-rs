macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! E {
    () => {
        deps!();
        pub (super) const E : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_THUESDAY) ;
    };
}

E!();