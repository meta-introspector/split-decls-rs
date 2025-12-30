// Generated macro for FixedTimespan (struct)
macro_rules! Depcrate_transitionsFixedTimespan {
() => {
// Module: crate::transitions
// Provides: {"FixedTimespan"}
// Dependencies: {}
# [doc = " An individual timespan with a fixed offset."] # [doc = ""] # [doc = " This mimics the `FixedTimespan` struct in `datetime::cal::zone`, except"] # [doc = " instead of “total offset” and “is DST” fields, it has separate UTC and"] # [doc = " DST fields. Also, the name is an owned `String` here instead of a slice."] # [derive (PartialEq , Debug , Clone)] pub struct FixedTimespan { # [doc = " The number of seconds offset from UTC during this timespan."] pub utc_offset : i64 , # [doc = " The number of *extra* daylight-saving seconds during this timespan."] pub dst_offset : i64 , # [doc = " The abbreviation in use during this timespan."] pub name : String , }
};
}
