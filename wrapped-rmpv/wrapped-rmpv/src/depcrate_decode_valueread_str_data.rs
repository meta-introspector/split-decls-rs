// Generated macro for read_str_data (function)
macro_rules! Depcrate_decode_valueread_str_data {
() => {
// Module: crate::decode::value
// Provides: {"read_str_data"}
// Dependencies: {}
fn read_str_data < R : Read > (rd : & mut R , len : usize , depth : u16) -> Result < Utf8String , Error > { let depth = super :: decrement_depth (depth) ? ; match String :: from_utf8 (read_bin_data (rd , len , depth) ?) { Ok (s) => Ok (Utf8String :: from (s)) , Err (err) => { let e = err . utf8_error () ; let s = Utf8String { s : Err ((err . into_bytes () , e)) , } ; Ok (s) } } }
};
}
