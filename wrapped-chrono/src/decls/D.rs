macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! D {
    () => {
        deps!();
        pub (super) const D : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_WEDNESDAY) ;
    };
}

D!()