macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! DC {
    () => {
        deps!();
        pub (super) const DC : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_WEDNESDAY) ;
    };
}

DC!()