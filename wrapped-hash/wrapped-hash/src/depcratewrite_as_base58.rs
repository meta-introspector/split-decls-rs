// Generated macro for write_as_base58 (function)
macro_rules! Depcratewrite_as_base58 {
() => {
// Module: crate
// Provides: {"write_as_base58"}
// Dependencies: {}
# [cfg (feature = "decode")] fn write_as_base58 (f : & mut fmt :: Formatter , h : & Hash) -> fmt :: Result { let mut out = [0u8 ; MAX_BASE58_LEN] ; let len = five8 :: encode_32 (& h . 0 , & mut out) as usize ; let as_str = unsafe { from_utf8_unchecked (& out [.. len]) } ; f . write_str (as_str) }
};
}
