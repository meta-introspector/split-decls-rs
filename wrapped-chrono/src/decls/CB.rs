macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! CB {
    () => {
        deps!();
        pub (super) const CB : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_THURSDAY) ;
    };
}

CB!();