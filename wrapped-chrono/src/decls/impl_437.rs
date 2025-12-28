macro_rules! deps {
    () => {
        NaiveDateTime!();
        NaiveDate!();
        ParseError!();
        Item!();
        Month!();
        Pad!();
        Fixed!();
        ParseResult!();
        Parsed!();
        Numeric!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        # [doc = " Parsing a `str` into a `NaiveDateTime` uses the same format,"] # [doc = " [`%Y-%m-%dT%H:%M:%S%.f`](crate::format::strftime), as in `Debug`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::{NaiveDateTime, NaiveDate};"] # [doc = ""] # [doc = " let dt = NaiveDate::from_ymd_opt(2015, 9, 18).unwrap().and_hms_opt(23, 56, 4).unwrap();"] # [doc = " assert_eq!(\"2015-09-18T23:56:04\".parse::<NaiveDateTime>(), Ok(dt));"] # [doc = ""] # [doc = " let dt = NaiveDate::from_ymd_opt(12345, 6, 7).unwrap().and_hms_milli_opt(7, 59, 59, 1_500).unwrap(); // leap second"] # [doc = " assert_eq!(\"+12345-6-7T7:59:60.5\".parse::<NaiveDateTime>(), Ok(dt));"] # [doc = ""] # [doc = " assert!(\"foo\".parse::<NaiveDateTime>().is_err());"] # [doc = " ```"] impl str :: FromStr for NaiveDateTime { type Err = ParseError ; fn from_str (s : & str) -> ParseResult < NaiveDateTime > { const ITEMS : & [Item < 'static >] = & [Item :: Numeric (Numeric :: Year , Pad :: Zero) , Item :: Space ("") , Item :: Literal ("-") , Item :: Numeric (Numeric :: Month , Pad :: Zero) , Item :: Space ("") , Item :: Literal ("-") , Item :: Numeric (Numeric :: Day , Pad :: Zero) , Item :: Space ("") , Item :: Literal ("T") , Item :: Numeric (Numeric :: Hour , Pad :: Zero) , Item :: Space ("") , Item :: Literal (":") , Item :: Numeric (Numeric :: Minute , Pad :: Zero) , Item :: Space ("") , Item :: Literal (":") , Item :: Numeric (Numeric :: Second , Pad :: Zero) , Item :: Fixed (Fixed :: Nanosecond) , Item :: Space ("") ,] ; let mut parsed = Parsed :: new () ; parse (& mut parsed , s , ITEMS . iter ()) ? ; parsed . to_naive_datetime_with_offset (0) } }
    };
}

impl_437!();