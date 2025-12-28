macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        # [doc = " The default value for a NaiveDate is 1st of January 1970."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use chrono::NaiveDate;"] # [doc = ""] # [doc = " let default_date = NaiveDate::default();"] # [doc = " assert_eq!(default_date, NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());"] # [doc = " ```"] impl Default for NaiveDate { fn default () -> Self { NaiveDate :: from_ymd_opt (1970 , 1 , 1) . unwrap () } }
    };
}

impl_365!()