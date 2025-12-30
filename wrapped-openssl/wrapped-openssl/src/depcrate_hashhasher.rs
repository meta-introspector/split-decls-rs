// Generated macro for Hasher (struct)
macro_rules! Depcrate_hashHasher {
() => {
// Module: crate::hash
// Provides: {"Hasher"}
// Dependencies: {}
# [doc = " Provides message digest (hash) computation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::hash::{Hasher, MessageDigest};"] # [doc = ""] # [doc = " # fn main() -> Result<(), Box<dyn std::error::Error>> {"] # [doc = " let data = [b\"\\x42\\xF4\", b\"\\x97\\xE0\"];"] # [doc = " let spec = b\"\\x7c\\x43\\x0f\\x17\\x8a\\xef\\xdf\\x14\\x87\\xfe\\xe7\\x14\\x4e\\x96\\x41\\xe2\";"] # [doc = " let mut h = Hasher::new(MessageDigest::md5())?;"] # [doc = " h.update(data[0])?;"] # [doc = " h.update(data[1])?;"] # [doc = " let res = h.finish()?;"] # [doc = " assert_eq!(&*res, spec);"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " Don't actually use MD5 and SHA-1 hashes, they're not secure anymore."] # [doc = ""] # [doc = " Don't ever hash passwords, use the functions in the `pkcs5` module or bcrypt/scrypt instead."] # [doc = ""] # [doc = " For extendable output functions (XOFs, i.e. SHAKE128/SHAKE256),"] # [doc = " you must use [`Hasher::finish_xof`] instead of [`Hasher::finish`]"] # [doc = " and provide a `buf` to store the hash. The hash will be as long as"] # [doc = " the `buf`."] pub struct Hasher { ctx : * mut ffi :: EVP_MD_CTX , md : * const ffi :: EVP_MD , type_ : MessageDigest , state : State , }
};
}
