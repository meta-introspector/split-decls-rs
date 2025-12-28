macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! FE {
    () => {
        deps!();
        pub (super) const FE : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_MONDAY) ;
    };
}

FE!();