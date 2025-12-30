// Generated macro for read_be_i64 (function)
macro_rules! Depcrate_offset_local_tz_info_parserread_be_i64 {
() => {
// Module: crate::offset::local::tz_info::parser
// Provides: {"read_be_i64"}
// Dependencies: {}
pub (crate) fn read_be_i64 (bytes : & [u8]) -> Result < i64 , Error > { if bytes . len () != 8 { return Err (Error :: InvalidSlice ("too short for i64")) ; } let mut buf = [0 ; 8] ; buf . copy_from_slice (bytes) ; Ok (i64 :: from_be_bytes (buf)) }
};
}
