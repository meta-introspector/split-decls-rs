macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! F {
    () => {
        deps!();
        pub (super) const F : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_MONDAY) ;
    };
}

F!();