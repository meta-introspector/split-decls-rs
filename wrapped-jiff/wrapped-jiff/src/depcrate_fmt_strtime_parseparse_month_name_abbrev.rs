// Generated macro for parse_month_name_abbrev (function)
macro_rules! Depcrate_fmt_strtime_parseparse_month_name_abbrev {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_month_name_abbrev"}
// Dependencies: {}
# [doc = " Like `parse_choice`, but specialized for month name abbreviation."] # [doc = ""] # [doc = " This exists because month name abbreviations are common and we can take"] # [doc = " advantage of the fact that they are all exactly three bytes."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_month_name_abbrev < 'i > (input : & 'i [u8] ,) -> Result < (usize , & 'i [u8]) , Error > { if input . len () < 3 { return Err (err ! ("expected to find a month name abbreviation, \
             but the remaining input, {input:?}, is too short \
             to contain one" , input = escape :: Bytes (input) ,)) ; } let (x , input) = input . split_at (3) ; let candidate = & [x [0] . to_ascii_lowercase () , x [1] . to_ascii_lowercase () , x [2] . to_ascii_lowercase () ,] ; let index = match candidate { b"jan" => 0 , b"feb" => 1 , b"mar" => 2 , b"apr" => 3 , b"may" => 4 , b"jun" => 5 , b"jul" => 6 , b"aug" => 7 , b"sep" => 8 , b"oct" => 9 , b"nov" => 10 , b"dec" => 11 , _ => { return Err (err ! ("expected to find month name abbreviation, but found \
                 {candidate:?} instead" , candidate = escape :: Bytes (x) ,)) } } ; Ok ((index , input)) }
};
}
