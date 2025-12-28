macro_rules! is_leap_year {
    () => {
        # [doc = " Check if a year is a leap year"] pub (crate) const fn is_leap_year (year : i32) -> bool { year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) }
    };
}

is_leap_year!();