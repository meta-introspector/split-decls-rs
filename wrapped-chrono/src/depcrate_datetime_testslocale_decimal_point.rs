// Generated macro for locale_decimal_point (function)
macro_rules! Depcrate_datetime_testslocale_decimal_point {
() => {
// Module: crate::datetime::tests
// Provides: {"locale_decimal_point"}
// Dependencies: {}
# [test] # [cfg (all (feature = "unstable-locales" , feature = "alloc"))] fn locale_decimal_point () { use crate :: Locale :: { ar_SY , nl_NL } ; let dt = Utc . with_ymd_and_hms (2018 , 9 , 5 , 18 , 58 , 0) . unwrap () . with_nanosecond (123456780) . unwrap () ; assert_eq ! (dt . format_localized ("%T%.f" , nl_NL) . to_string () , "18:58:00,123456780") ; assert_eq ! (dt . format_localized ("%T%.3f" , nl_NL) . to_string () , "18:58:00,123") ; assert_eq ! (dt . format_localized ("%T%.6f" , nl_NL) . to_string () , "18:58:00,123456") ; assert_eq ! (dt . format_localized ("%T%.9f" , nl_NL) . to_string () , "18:58:00,123456780") ; assert_eq ! (dt . format_localized ("%T%.f" , ar_SY) . to_string () , "18:58:00.123456780") ; assert_eq ! (dt . format_localized ("%T%.3f" , ar_SY) . to_string () , "18:58:00.123") ; assert_eq ! (dt . format_localized ("%T%.6f" , ar_SY) . to_string () , "18:58:00.123456") ; assert_eq ! (dt . format_localized ("%T%.9f" , ar_SY) . to_string () , "18:58:00.123456780") ; }
};
}
