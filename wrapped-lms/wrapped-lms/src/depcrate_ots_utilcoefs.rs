// Generated macro for coefs (function)
macro_rules! Depcrate_ots_utilcoefs {
() => {
// Module: crate::ots::util
// Provides: {"coefs"}
// Dependencies: {}
# [doc = " Returns an iterator over the w-bit Winternitz coefficients of the inout bytes"] # [doc = " Implements the Coef function from section 3.1.3 of RFC8554"] # [doc = " https://datatracker.ietf.org/doc/html/rfc8554#section-3.1.3"] pub (crate) fn coefs < 'a > (bytes : impl IntoIterator < Item = & 'a u8 > , w : usize ,) -> impl Iterator < Item = u8 > { let mask : u8 = match w { 1 => 0x01 , 2 => 0x03 , 4 => 0x0f , 8 => 0xff , _ => panic ! ("invalid bit width: {w}") , } ; let entries_per_byte : usize = 8 / w ; bytes . into_iter () . cloned () . flat_map (move | byte | (0 .. entries_per_byte) . map (move | i | (byte >> (8 - w - i * w)) & mask)) }
};
}
