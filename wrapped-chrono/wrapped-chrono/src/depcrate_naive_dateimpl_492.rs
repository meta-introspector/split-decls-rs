// Generated macro for impl_492 (impl)
macro_rules! Depcrate_naive_dateimpl_492 {
() => {
// Module: crate::naive::date
// Provides: {"impl_492"}
// Dependencies: {}
# [doc = " Parsing a `str` into a `NaiveDate` uses the same format,"] # [doc = " [`%Y-%m-%d`](crate::format::strftime), as in `Debug` and `Display`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use chrono::NaiveDate;"] # [doc = ""] # [doc = " let d = NaiveDate::from_ymd_opt(2015, 9, 18).unwrap();"] # [doc = " assert_eq!(\"2015-09-18\".parse::<NaiveDate>(), Ok(d));"] # [doc = ""] # [doc = " let d = NaiveDate::from_ymd_opt(12345, 6, 7).unwrap();"] # [doc = " assert_eq!(\"+12345-6-7\".parse::<NaiveDate>(), Ok(d));"] # [doc = ""] # [doc = " assert!(\"foo\".parse::<NaiveDate>().is_err());"] # [doc = " ```"] impl str :: FromStr for NaiveDate { type Err = ParseError ; fn from_str (s : & str) -> ParseResult < NaiveDate > { const ITEMS : & [Item < 'static >] = & [Item :: Numeric (Numeric :: Year , Pad :: Zero) , Item :: Space ("") , Item :: Literal ("-") , Item :: Numeric (Numeric :: Month , Pad :: Zero) , Item :: Space ("") , Item :: Literal ("-") , Item :: Numeric (Numeric :: Day , Pad :: Zero) , Item :: Space ("") ,] ; let mut parsed = Parsed :: new () ; parse (& mut parsed , s , ITEMS . iter ()) ? ; parsed . to_naive_date () } }
};
}
