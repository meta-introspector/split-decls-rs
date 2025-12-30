// Generated macro for hash (function)
macro_rules! Depcrate_hashhash {
() => {
// Module: crate::hash
// Provides: {"hash"}
// Dependencies: {}
# [doc = " Computes the hash of the `data` with the non-XOF hasher `t`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " use openssl::hash::{hash, MessageDigest};"] # [doc = ""] # [doc = " let data = b\"\\x42\\xF4\\x97\\xE0\";"] # [doc = " let spec = b\"\\x7c\\x43\\x0f\\x17\\x8a\\xef\\xdf\\x14\\x87\\xfe\\xe7\\x14\\x4e\\x96\\x41\\xe2\";"] # [doc = " let res = hash(MessageDigest::md5(), data)?;"] # [doc = " assert_eq!(&*res, spec);"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub fn hash (t : MessageDigest , data : & [u8]) -> Result < DigestBytes , ErrorStack > { let mut h = Hasher :: new (t) ? ; h . update (data) ? ; h . finish () }
};
}
