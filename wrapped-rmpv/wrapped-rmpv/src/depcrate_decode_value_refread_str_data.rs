// Generated macro for read_str_data (function)
macro_rules! Depcrate_decode_value_refread_str_data {
() => {
// Module: crate::decode::value_ref
// Provides: {"read_str_data"}
// Dependencies: {}
fn read_str_data < 'a , R > (rd : & mut R , len : usize , depth : u16) -> Result < Utf8StringRef < 'a > , Error > where R : BorrowRead < 'a > { let depth = super :: decrement_depth (depth) ? ; let buf = read_bin_data (rd , len , depth) ? ; match str :: from_utf8 (buf) { Ok (s) => Ok (Utf8StringRef :: from (s)) , Err (err) => { let s = Utf8StringRef { s : Err ((buf , err)) , } ; Ok (s) } } }
};
}
