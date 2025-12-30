// Generated macro for impl_173 (impl)
macro_rules! Depcrate_hmacimpl_173 {
() => {
// Module: crate::hmac
// Provides: {"impl_173"}
// Dependencies: {}
impl HmacSha512 { # [doc = " Computes the HMAC-SHA512 of `data` as a one-shot operation."] pub fn mac (key : & [u8] , data : & [u8]) -> [u8 ; 64] { hmac :: < 64 , Sha512 > (key , data) } # [doc = " Creates a new HMAC-SHA512 operation from a fixed-size key."] pub fn new (key : & [u8 ; 64]) -> Self { Self (Hmac :: new (key)) } # [doc = " Creates a new HMAC-SHA512 operation from a variable-length key."] pub fn new_from_slice (key : & [u8]) -> Self { Self (Hmac :: new_from_slice (key)) } # [doc = " Hashes the provided input into the HMAC operation."] pub fn update (& mut self , data : & [u8]) { self . 0 . update (data) } # [doc = " Computes the final HMAC value, consuming the object."] pub fn digest (self) -> [u8 ; 64] { self . 0 . digest () } # [doc = " Checks that the provided tag value matches the computed HMAC value."] pub fn verify_slice (self , tag : & [u8]) -> Result < () , InvalidSignatureError > { self . 0 . verify_slice (tag) } # [doc = " Checks that the provided tag value matches the computed HMAC value."] pub fn verify (self , tag : & [u8 ; 64]) -> Result < () , InvalidSignatureError > { self . 0 . verify (tag) } # [doc = " Checks that the provided tag value matches the computed HMAC, truncated to the input tag's"] # [doc = " length."] # [doc = ""] # [doc = " Truncating an HMAC reduces the security of the construction. Callers must ensure `tag`'s"] # [doc = " length matches the desired HMAC length and security level."] pub fn verify_truncated_left (self , tag : & [u8]) -> Result < () , InvalidSignatureError > { self . 0 . verify_truncated_left (tag) } }
};
}
