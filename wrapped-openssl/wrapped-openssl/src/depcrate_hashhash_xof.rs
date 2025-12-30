// Generated macro for hash_xof (function)
macro_rules! Depcrate_hashhash_xof {
() => {
// Module: crate::hash
// Provides: {"hash_xof"}
// Dependencies: {}
# [doc = " Computes the hash of the `data` with the XOF hasher `t` and stores it in `buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::hash::{hash_xof, MessageDigest};"] # [doc = ""] # [doc = " let data = b\"\\x41\\x6c\\x6c\\x20\\x79\\x6f\\x75\\x72\\x20\\x62\\x61\\x73\\x65\\x20\\x61\\x72\\x65\\x20\\x62\\x65\\x6c\\x6f\\x6e\\x67\\x20\\x74\\x6f\\x20\\x75\\x73\";"] # [doc = " let spec = b\"\\x49\\xd0\\x69\\x7f\\xf5\\x08\\x11\\x1d\\x8b\\x84\\xf1\\x5e\\x46\\xda\\xf1\\x35\";"] # [doc = " let mut buf = vec![0; 16];"] # [doc = " hash_xof(MessageDigest::shake_128(), data, buf.as_mut_slice()).unwrap();"] # [doc = " assert_eq!(buf, spec);"] # [doc = " ```"] # [doc = ""] # [cfg (any (ossl111 , awslc))] pub fn hash_xof (t : MessageDigest , data : & [u8] , buf : & mut [u8]) -> Result < () , ErrorStack > { let mut h = Hasher :: new (t) ? ; h . update (data) ? ; h . finish_xof (buf) }
};
}
