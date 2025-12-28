macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        # [doc = " Subtracts another `NaiveDate` from the current date."] # [doc = " Returns a `TimeDelta` of integral numbers."] # [doc = ""] # [doc = " This does not overflow or underflow at all,"] # [doc = " as all possible output fits in the range of `TimeDelta`."] # [doc = ""] # [doc = " The implementation is a wrapper around"] # [doc = " [`NaiveDate::signed_duration_since`](#method.signed_duration_since)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{NaiveDate, TimeDelta};"] # [doc = ""] # [doc = " let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();"] # [doc = ""] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2014, 1, 1), TimeDelta::zero());"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 12, 31), TimeDelta::try_days(1).unwrap());"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2014, 1, 2), TimeDelta::try_days(-1).unwrap());"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 9, 23), TimeDelta::try_days(100).unwrap());"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - from_ymd(2013, 1, 1), TimeDelta::try_days(365).unwrap());"] # [doc = " assert_eq!("] # [doc = "     from_ymd(2014, 1, 1) - from_ymd(2010, 1, 1),"] # [doc = "     TimeDelta::try_days(365 * 4 + 1).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     from_ymd(2014, 1, 1) - from_ymd(1614, 1, 1),"] # [doc = "     TimeDelta::try_days(365 * 400 + 97).unwrap()"] # [doc = " );"] # [doc = " ```"] impl Sub < NaiveDate > for NaiveDate { type Output = TimeDelta ; # [inline] fn sub (self , rhs : NaiveDate) -> TimeDelta { self . signed_duration_since (rhs) } }
    };
}

impl_349!();