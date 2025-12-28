macro_rules! deps {
    () => {
        Months!();
        NaiveDateTime!();
        NaiveDate!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        # [doc = " Subtract `Months` from `NaiveDateTime`."] # [doc = ""] # [doc = " The result will be clamped to valid days in the resulting month, see"] # [doc = " [`NaiveDateTime::checked_sub_months`] for details."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the resulting date would be out of range."] # [doc = " Consider using [`NaiveDateTime::checked_sub_months`] to get an `Option` instead."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Months, NaiveDate};"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(01, 00, 00).unwrap()"] # [doc = "         - Months::new(11),"] # [doc = "     NaiveDate::from_ymd_opt(2013, 02, 01).unwrap().and_hms_opt(01, 00, 00).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(00, 02, 00).unwrap()"] # [doc = "         - Months::new(12),"] # [doc = "     NaiveDate::from_ymd_opt(2013, 01, 01).unwrap().and_hms_opt(00, 02, 00).unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     NaiveDate::from_ymd_opt(2014, 01, 01).unwrap().and_hms_opt(00, 00, 03).unwrap()"] # [doc = "         - Months::new(13),"] # [doc = "     NaiveDate::from_ymd_opt(2012, 12, 01).unwrap().and_hms_opt(00, 00, 03).unwrap()"] # [doc = " );"] # [doc = " ```"] impl Sub < Months > for NaiveDateTime { type Output = NaiveDateTime ; fn sub (self , rhs : Months) -> Self :: Output { self . checked_sub_months (rhs) . expect ("`NaiveDateTime - Months` out of range") } }
    };
}

impl_430!()