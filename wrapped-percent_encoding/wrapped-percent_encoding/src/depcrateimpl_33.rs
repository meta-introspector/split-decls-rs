// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > PercentDecode < 'a > { # [doc = " If the percent-decoding is different from the input, return it as a new bytes vector."] # [cfg (feature = "alloc")] fn if_any (& self) -> Option < Vec < u8 > > { let mut bytes_iter = self . bytes . clone () ; while bytes_iter . any (| & b | b == b'%') { if let Some (decoded_byte) = after_percent_sign (& mut bytes_iter) { let initial_bytes = self . bytes . as_slice () ; let unchanged_bytes_len = initial_bytes . len () - bytes_iter . len () - 3 ; let mut decoded = initial_bytes [.. unchanged_bytes_len] . to_owned () ; decoded . push (decoded_byte) ; decoded . extend (PercentDecode { bytes : bytes_iter }) ; return Some (decoded) ; } } None } # [doc = " Decode the result of percent-decoding as UTF-8."] # [doc = ""] # [doc = " This is return `Err` when the percent-decoded bytes are not well-formed in UTF-8."] # [cfg (feature = "alloc")] pub fn decode_utf8 (self) -> Result < Cow < 'a , str > , str :: Utf8Error > { match self . clone () . into () { Cow :: Borrowed (bytes) => match str :: from_utf8 (bytes) { Ok (s) => Ok (s . into ()) , Err (e) => Err (e) , } , Cow :: Owned (bytes) => match String :: from_utf8 (bytes) { Ok (s) => Ok (s . into ()) , Err (e) => Err (e . utf8_error ()) , } , } } # [doc = " Decode the result of percent-decoding as UTF-8, lossily."] # [doc = ""] # [doc = " Invalid UTF-8 percent-encoded byte sequences will be replaced � U+FFFD,"] # [doc = " the replacement character."] # [cfg (feature = "alloc")] pub fn decode_utf8_lossy (self) -> Cow < 'a , str > { decode_utf8_lossy (self . clone () . into ()) } }
};
}
