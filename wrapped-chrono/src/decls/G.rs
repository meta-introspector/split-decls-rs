macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! G {
    () => {
        deps!();
        pub (super) const G : YearFlags = YearFlags (COMMON_YEAR | YEAR_STARTS_AFTER_SUNDAY) ;
    };
}

G!()