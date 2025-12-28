macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        # [doc = " Add `TimeDelta` to `NaiveDate`."] # [doc = ""] # [doc = " This discards the fractional days in `TimeDelta`, rounding to the closest integral number of"] # [doc = " days towards `TimeDelta::zero()`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDate::checked_add_signed`] to get an `Option` instead."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{NaiveDate, TimeDelta};"] # [doc = ""] # [doc = " let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();"] # [doc = ""] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::zero(), from_ymd(2014, 1, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::try_seconds(86399).unwrap(), from_ymd(2014, 1, 1));"] # [doc = " assert_eq!("] # [doc = "     from_ymd(2014, 1, 1) + TimeDelta::try_seconds(-86399).unwrap(),"] # [doc = "     from_ymd(2014, 1, 1)"] # [doc = " );"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::try_days(1).unwrap(), from_ymd(2014, 1, 2));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::try_days(-1).unwrap(), from_ymd(2013, 12, 31));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + TimeDelta::try_days(364).unwrap(), from_ymd(2014, 12, 31));"] # [doc = " assert_eq!("] # [doc = "     from_ymd(2014, 1, 1) + TimeDelta::try_days(365 * 4 + 1).unwrap(),"] # [doc = "     from_ymd(2018, 1, 1)"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     from_ymd(2014, 1, 1) + TimeDelta::try_days(365 * 400 + 97).unwrap(),"] # [doc = "     from_ymd(2414, 1, 1)"] # [doc = " );"] # [doc = " ```"] # [doc = ""] # [doc = " [`NaiveDate::checked_add_signed`]: crate::NaiveDate::checked_add_signed"] impl Add < TimeDelta > for NaiveDate { type Output = NaiveDate ; # [inline] fn add (self , rhs : TimeDelta) -> NaiveDate { self . checked_add_signed (rhs) . expect ("`NaiveDate + TimeDelta` overflowed") } }
    };
}

impl_341!()