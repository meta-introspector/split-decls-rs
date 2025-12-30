// Generated macro for impl_189 (impl)
macro_rules! Depcrate_utilimpl_189 {
() => {
// Module: crate::util
// Provides: {"impl_189"}
// Dependencies: {}
impl DecimalFormatter { # [doc = " Discovered via `u64::MAX.to_string().len()`."] const MAX_U64_LEN : usize = 20 ; # [doc = " Create a new decimal formatter for the given 64-bit unsigned integer."] pub (crate) fn new (mut n : u64) -> DecimalFormatter { let mut buf = [0 ; Self :: MAX_U64_LEN] ; let mut i = buf . len () ; loop { i -= 1 ; let digit = u8 :: try_from (n % 10) . unwrap () ; n /= 10 ; buf [i] = b'0' + digit ; if n == 0 { break ; } } DecimalFormatter { buf , start : i } } # [doc = " Return the decimal formatted as an ASCII byte string."] pub (crate) fn as_bytes (& self) -> & [u8] { & self . buf [self . start ..] } }
};
}
