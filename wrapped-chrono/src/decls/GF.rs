macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! GF {
    () => {
        deps!();
        pub (super) const GF : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_SUNDAY) ;
    };
}

GF!();