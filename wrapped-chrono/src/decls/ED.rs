macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! ED {
    () => {
        deps!();
        pub (super) const ED : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_THUESDAY) ;
    };
}

ED!()