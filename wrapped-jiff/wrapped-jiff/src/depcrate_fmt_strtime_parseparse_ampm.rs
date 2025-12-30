// Generated macro for parse_ampm (function)
macro_rules! Depcrate_fmt_strtime_parseparse_ampm {
() => {
// Module: crate::fmt::strtime::parse
// Provides: {"parse_ampm"}
// Dependencies: {}
# [doc = " Like `parse_choice`, but specialized for AM/PM."] # [doc = ""] # [doc = " This exists because AM/PM is common and we can take advantage of the fact"] # [doc = " that they are both exactly two bytes."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn parse_ampm < 'i > (input : & 'i [u8]) -> Result < (usize , & 'i [u8]) , Error > { if input . len () < 2 { return Err (err ! ("expected to find AM or PM, \
             but the remaining input, {input:?}, is too short \
             to contain one" , input = escape :: Bytes (input) ,)) ; } let (x , input) = input . split_at (2) ; let candidate = & [x [0] . to_ascii_lowercase () , x [1] . to_ascii_lowercase ()] ; let index = match candidate { b"am" => 0 , b"pm" => 1 , _ => { return Err (err ! ("expected to find AM or PM, but found \
                {candidate:?} instead" , candidate = escape :: Bytes (x) ,)) } } ; Ok ((index , input)) }
};
}
