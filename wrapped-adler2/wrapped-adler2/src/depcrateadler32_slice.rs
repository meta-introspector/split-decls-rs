// Generated macro for adler32_slice (function)
macro_rules! Depcrateadler32_slice {
() => {
// Module: crate
// Provides: {"adler32_slice"}
// Dependencies: {}
# [doc = " Calculates the Adler-32 checksum of a byte slice."] # [doc = ""] # [doc = " This is a convenience function around the [`Adler32`] type."] # [doc = ""] # [doc = " [`Adler32`]: struct.Adler32.html"] pub fn adler32_slice (data : & [u8]) -> u32 { let mut h = Adler32 :: new () ; h . write_slice (data) ; h . checksum () }
};
}
