macro_rules! deps {
    () => {
        Months!();
        NaiveDate!();
        NaiveDateTime!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [doc = " Add `Months` to `NaiveDateTime`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see `checked_add_months` for"] # [doc = " details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using `checked_add_months` to get an `Option` instead."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Months, NaiveDate};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap() + Months::new(1),"] # [doc = "     NaiveDate::from_ymd_opt(2014, 2, 1).unwrap().and_hms_opt(1, 0, 0).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 2, 0).unwrap()"] # [doc = "         + Months::new(11),"] # [doc = "     NaiveDate::from_ymd_opt(2014, 12, 1).unwrap().and_hms_opt(0, 2, 0).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 0, 3).unwrap()"] # [doc = "         + Months::new(12),"] # [doc = "     NaiveDate::from_ymd_opt(2015, 1, 1).unwrap().and_hms_opt(0, 0, 3).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 1, 1).unwrap().and_hms_opt(0, 0, 4).unwrap()"] # [doc = "         + Months::new(13),"] # [doc = "     NaiveDate::from_ymd_opt(2015, 2, 1).unwrap().and_hms_opt(0, 0, 4).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 1, 31).unwrap().and_hms_opt(0, 5, 0).unwrap()"] # [doc = "         + Months::new(1),"] # [doc = "     NaiveDate::from_ymd_opt(2014, 2, 28).unwrap().and_hms_opt(0, 5, 0).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2020, 1, 31).unwrap().and_hms_opt(6, 0, 0).unwrap()"] # [doc = "         + Months::new(1),"] # [doc = "     NaiveDate::from_ymd_opt(2020, 2, 29).unwrap().and_hms_opt(6, 0, 0).unwrap()"] # [doc = " );"] # [doc = " ```"] impl Add < Months > for NaiveDateTime { type Output = NaiveDateTime ; fn add (self , rhs : Months) -> Self :: Output { self . checked_add_months (rhs) . expect ("`NaiveDateTime + Months` out of range") } }
    };
}

impl_424!()