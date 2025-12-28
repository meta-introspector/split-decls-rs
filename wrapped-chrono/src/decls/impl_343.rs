macro_rules! deps {
    () => {
        Months!();
        NaiveDate!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        # [doc = " Add `Months` to `NaiveDate`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see `checked_add_months` for"] # [doc = " details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_add_months` to get an `Option` instead."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Months, NaiveDate};"] # [doc = ""] # [doc = " let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();"] # [doc = ""] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + Months::new(1), from_ymd(2014, 2, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + Months::new(11), from_ymd(2014, 12, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + Months::new(12), from_ymd(2015, 1, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) + Months::new(13), from_ymd(2015, 2, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 31) + Months::new(1), from_ymd(2014, 2, 28));"] # [doc = " assert_eq!(from_ymd(2020, 1, 31) + Months::new(1), from_ymd(2020, 2, 29));"] # [doc = " ```"] impl Add < Months > for NaiveDate { type Output = NaiveDate ; fn add (self , months : Months) -> Self :: Output { self . checked_add_months (months) . expect ("`NaiveDate + Months` out of range") } }
    };
}

impl_343!();