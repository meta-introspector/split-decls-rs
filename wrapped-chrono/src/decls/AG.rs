macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! AG {
    () => {
        deps!();
        pub (super) const AG : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_SATURDAY) ;
    };
}

AG!();