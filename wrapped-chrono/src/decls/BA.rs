macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! BA {
    () => {
        deps!();
        pub (super) const BA : YearFlags = YearFlags (LEAP_YEAR | YEAR_STARTS_AFTER_FRIDAY) ;
    };
}

BA!()