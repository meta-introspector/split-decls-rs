// Generated macro for to_bytes_be_with_len (function)
macro_rules! Depcrate_ffdheto_bytes_be_with_len {
() => {
// Module: crate::ffdhe
// Provides: {"to_bytes_be_with_len"}
// Dependencies: {}
fn to_bytes_be_with_len (n : BigUint , len_bytes : usize) -> Vec < u8 > { let mut bytes = n . to_bytes_le () ; bytes . resize (len_bytes , 0) ; bytes . reverse () ; bytes }
};
}
