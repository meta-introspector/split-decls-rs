macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! C {
    () => {
        deps!();
        pub (super) const C : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_THURSDAY) ;
    };
}

C!();