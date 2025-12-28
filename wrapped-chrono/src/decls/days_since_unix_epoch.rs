macro_rules! deps {
    () => {
        Month!();
    };
}

macro_rules! days_since_unix_epoch {
    () => {
        deps!();
        # [doc = " Compute the number of days since Unix epoch (`1970-01-01T00:00:00Z`)."] # [doc = ""] # [doc = " ## Inputs"] # [doc = ""] # [doc = " * `year`: Year"] # [doc = " * `month`: Month in `[1, 12]`"] # [doc = " * `month_day`: Day of the month in `[1, 31]`"] pub (crate) const fn days_since_unix_epoch (year : i32 , month : usize , month_day : i64) -> i64 { let is_leap_year = is_leap_year (year) ; let year = year as i64 ; let mut result = (year - 1970) * 365 ; if year >= 1970 { result += (year - 1968) / 4 ; result -= (year - 1900) / 100 ; result += (year - 1600) / 400 ; if is_leap_year && month < 3 { result -= 1 ; } } else { result += (year - 1972) / 4 ; result -= (year - 2000) / 100 ; result += (year - 2000) / 400 ; if is_leap_year && month >= 3 { result += 1 ; } } result += CUMUL_DAY_IN_MONTHS_NORMAL_YEAR [month - 1] + month_day - 1 ; result }
    };
}

days_since_unix_epoch!();