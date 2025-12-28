macro_rules! deps {
    () => {
        YearFlags!();
        IsoWeek!();
        NaiveDate!();
        Datelike!();
        Weekday!();
    };
}

macro_rules! impl_478 {
    () => {
        deps!();
        impl IsoWeek { # [doc = " Returns the corresponding `IsoWeek` from the year and the `Of` internal value."] pub (super) fn from_yof (year : i32 , ordinal : u32 , year_flags : YearFlags) -> Self { let rawweek = (ordinal + year_flags . isoweek_delta ()) / 7 ; let (year , week) = if rawweek < 1 { let prevlastweek = YearFlags :: from_year (year - 1) . nisoweeks () ; (year - 1 , prevlastweek) } else { let lastweek = year_flags . nisoweeks () ; if rawweek > lastweek { (year + 1 , 1) } else { (year , rawweek) } } ; let flags = YearFlags :: from_year (year) ; IsoWeek { ywf : (year << 10) | (week << 4) as i32 | i32 :: from (flags . 0) } } # [doc = " Returns the year number for this ISO week."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Datelike, NaiveDate, Weekday};"] # [doc = ""] # [doc = " let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();"] # [doc = " assert_eq!(d.iso_week().year(), 2015);"] # [doc = " ```"] # [doc = ""] # [doc = " This year number might not match the calendar year number."] # [doc = " Continuing the example..."] # [doc = ""] # [doc = " ```"] # [doc = " # use chrono::{NaiveDate, Datelike, Weekday};"] # [doc = " # let d = NaiveDate::from_isoywd_opt(2015, 1, Weekday::Mon).unwrap();"] # [doc = " assert_eq!(d.year(), 2014);"] # [doc = " assert_eq!(d, NaiveDate::from_ymd_opt(2014, 12, 29).unwrap());"] # [doc = " ```"] # [inline] pub const fn year (& self) -> i32 { self . ywf >> 10 } # [doc = " Returns the ISO week number starting from 1."] # [doc = ""] # [doc = " The return value ranges from 1 to 53. (The last week of year differs by years.)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Datelike, NaiveDate, Weekday};"] # [doc = ""] # [doc = " let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();"] # [doc = " assert_eq!(d.iso_week().week(), 15);"] # [doc = " ```"] # [inline] pub const fn week (& self) -> u32 { ((self . ywf >> 4) & 0x3f) as u32 } # [doc = " Returns the ISO week number starting from 0."] # [doc = ""] # [doc = " The return value ranges from 0 to 52. (The last week of year differs by years.)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{Datelike, NaiveDate, Weekday};"] # [doc = ""] # [doc = " let d = NaiveDate::from_isoywd_opt(2015, 15, Weekday::Mon).unwrap();"] # [doc = " assert_eq!(d.iso_week().week0(), 14);"] # [doc = " ```"] # [inline] pub const fn week0 (& self) -> u32 { ((self . ywf >> 4) & 0x3f) as u32 - 1 } }
    };
}

impl_478!();