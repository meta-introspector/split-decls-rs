// Generated macro for read_be_i32 (function)
macro_rules! Depcrate_offset_local_tz_info_parserread_be_i32 {
() => {
// Module: crate::offset::local::tz_info::parser
// Provides: {"read_be_i32"}
// Dependencies: {}
pub (crate) fn read_be_i32 (bytes : & [u8]) -> Result < i32 , Error > { if bytes . len () != 4 { return Err (Error :: InvalidSlice ("too short for i32")) ; } let mut buf = [0 ; 4] ; buf . copy_from_slice (bytes) ; Ok (i32 :: from_be_bytes (buf)) }
};
}
