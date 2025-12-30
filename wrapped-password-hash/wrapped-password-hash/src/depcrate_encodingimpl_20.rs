// Generated macro for impl_20 (impl)
macro_rules! Depcrate_encodingimpl_20 {
() => {
// Module: crate::encoding
// Provides: {"impl_20"}
// Dependencies: {}
impl Encoding { # [doc = " Decode a Base64 string into the provided destination buffer."] pub fn decode (self , src : impl AsRef < [u8] > , dst : & mut [u8]) -> Result < & [u8] , B64Error > { match self { Self :: B64 => B64 :: decode (src , dst) , Self :: Bcrypt => Base64Bcrypt :: decode (src , dst) , Self :: Crypt => Base64Crypt :: decode (src , dst) , Self :: ShaCrypt => Base64ShaCrypt :: decode (src , dst) , } } # [doc = " Encode the input byte slice as Base64."] # [doc = ""] # [doc = " Writes the result into the provided destination slice, returning an"] # [doc = " ASCII-encoded Base64 string value."] pub fn encode < 'a > (self , src : & [u8] , dst : & 'a mut [u8]) -> Result < & 'a str , B64Error > { match self { Self :: B64 => B64 :: encode (src , dst) , Self :: Bcrypt => Base64Bcrypt :: encode (src , dst) , Self :: Crypt => Base64Crypt :: encode (src , dst) , Self :: ShaCrypt => Base64ShaCrypt :: encode (src , dst) , } . map_err (Into :: into) } # [doc = " Get the length of Base64 produced by encoding the given bytes."] pub fn encoded_len (self , bytes : & [u8]) -> usize { match self { Self :: B64 => B64 :: encoded_len (bytes) , Self :: Bcrypt => Base64Bcrypt :: encoded_len (bytes) , Self :: Crypt => Base64Crypt :: encoded_len (bytes) , Self :: ShaCrypt => Base64ShaCrypt :: encoded_len (bytes) , } } }
};
}
