// Generated macro for parse_weekday_abbrev (function)
macro_rules! Depcrate_fmt_strtime_parseparse_weekday_abbrev {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_weekday_abbrev"}
// Dependencies: {}
# [doc = " Like `parse_choice`, but specialized for weekday abbreviation."] # [doc = ""] # [doc = " This exists because weekday abbreviations are common and we can take"] # [doc = " advantage of the fact that they are all exactly three bytes."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_weekday_abbrev < 'i > (input : & 'i [u8] ,) -> Result < (usize , & 'i [u8]) , Error > { if input . len () < 3 { return Err (err ! ("expected to find a weekday abbreviation, \
             but the remaining input, {input:?}, is too short \
             to contain one" , input = escape :: Bytes (input) ,)) ; } let (x , input) = input . split_at (3) ; let candidate = & [x [0] . to_ascii_lowercase () , x [1] . to_ascii_lowercase () , x [2] . to_ascii_lowercase () ,] ; let index = match candidate { b"sun" => 0 , b"mon" => 1 , b"tue" => 2 , b"wed" => 3 , b"thu" => 4 , b"fri" => 5 , b"sat" => 6 , _ => { return Err (err ! ("expected to find weekday abbreviation, but found \
                {candidate:?} instead" , candidate = escape :: Bytes (x) ,)) } } ; Ok ((index , input)) }
};
}
