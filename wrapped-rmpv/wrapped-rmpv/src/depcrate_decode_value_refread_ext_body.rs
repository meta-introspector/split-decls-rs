// Generated macro for read_ext_body (function)
macro_rules! Depcrate_decode_value_refread_ext_body {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_ext_body"}
// Dependencies: {}
fn read_ext_body < 'a , R > (rd : & mut R , len : usize , depth : u16) -> Result < (i8 , & 'a [u8]) , Error > where R : BorrowRead < 'a > { let depth = super :: decrement_depth (depth) ? ; let ty = rd . read_data_i8 () ? ; let buf = read_bin_data (rd , len , depth) ? ; Ok ((ty , buf)) }
};
}
