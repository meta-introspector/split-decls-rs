// Generated macro for read_ext_body (function)
macro_rules! Depcrate_decode_valueread_ext_body {
() => {
// Module: crate::decode::value
// Provides: {"read_ext_body"}
// Dependencies: {}
fn read_ext_body < R : Read > (rd : & mut R , len : usize , depth : u16) -> Result < (i8 , Vec < u8 >) , Error > { let depth = super :: decrement_depth (depth) ? ; let ty = rd . read_data_i8 () ? ; let vec = read_bin_data (rd , len , depth) ? ; Ok ((ty , vec)) }
};
}
