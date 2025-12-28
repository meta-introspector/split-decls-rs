macro_rules! deps {
    () => {
        NaiveDate!();
        Months!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        # [doc = " Subtract `Months` from `NaiveDate`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see `checked_sub_months` for"] # [doc = " details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `NaiveDate::checked_sub_months` to get an `Option` instead."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Months, NaiveDate};"] # [doc = ""] # [doc = " let from_ymd = |y, m, d| NaiveDate::from_ymd_opt(y, m, d).unwrap();"] # [doc = ""] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - Months::new(11), from_ymd(2013, 2, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - Months::new(12), from_ymd(2013, 1, 1));"] # [doc = " assert_eq!(from_ymd(2014, 1, 1) - Months::new(13), from_ymd(2012, 12, 1));"] # [doc = " ```"] impl Sub < Months > for NaiveDate { type Output = NaiveDate ; fn sub (self , months : Months) -> Self :: Output { self . checked_sub_months (months) . expect ("`NaiveDate - Months` out of range") } }
    };
}

impl_344!()