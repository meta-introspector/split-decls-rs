// Generated macro for write (module)
macro_rules! Depcratewrite {
() => {
// Module: crate
// Provides: {"write"}
// Dependencies: {}
# [doc = " A module for writing LEB128-encoded signed and unsigned integers."] pub mod write { use super :: { low_bits_of_u64 , CONTINUATION_BIT } ; use std :: io ; # [doc = " Write `val` to the `std::io::Write` stream `w` as an unsigned LEB128 value."] # [doc = ""] # [doc = " On success, return the number of bytes written to `w`."] pub fn unsigned < W > (w : & mut W , mut val : u64) -> Result < usize , io :: Error > where W : ? Sized + io :: Write , { let mut bytes_written = 0 ; loop { let mut byte = low_bits_of_u64 (val) ; val >>= 7 ; if val != 0 { byte |= CONTINUATION_BIT ; } let buf = [byte] ; w . write_all (& buf) ? ; bytes_written += 1 ; if val == 0 { return Ok (bytes_written) ; } } } # [doc = " Write `val` to the `std::io::Write` stream `w` as a signed LEB128 value."] # [doc = ""] # [doc = " On success, return the number of bytes written to `w`."] pub fn signed < W > (w : & mut W , mut val : i64) -> Result < usize , io :: Error > where W : ? Sized + io :: Write , { let mut bytes_written = 0 ; loop { let mut byte = val as u8 ; val >>= 6 ; let done = val == 0 || val == - 1 ; if done { byte &= ! CONTINUATION_BIT ; } else { val >>= 1 ; byte |= CONTINUATION_BIT ; } let buf = [byte] ; w . write_all (& buf) ? ; bytes_written += 1 ; if done { return Ok (bytes_written) ; } } } }
};
}
