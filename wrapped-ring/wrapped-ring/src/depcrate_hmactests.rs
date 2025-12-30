// Generated macro for tests (module)
macro_rules! Depcrate_hmactests {
() => {
// Module: crate::hmac
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { hmac , rand } ; # [test] pub fn hmac_signing_key_coverage () { let rng = rand :: SystemRandom :: new () ; const HELLO_WORLD_GOOD : & [u8] = b"hello, world" ; const HELLO_WORLD_BAD : & [u8] = b"hello, worle" ; for algorithm in & [hmac :: HMAC_SHA1_FOR_LEGACY_USE_ONLY , hmac :: HMAC_SHA256 , hmac :: HMAC_SHA384 , hmac :: HMAC_SHA512 ,] { let key = hmac :: Key :: generate (* algorithm , & rng) . unwrap () ; let tag = hmac :: sign (& key , HELLO_WORLD_GOOD) ; assert ! (hmac :: verify (& key , HELLO_WORLD_GOOD , tag . as_ref ()) . is_ok ()) ; assert ! (hmac :: verify (& key , HELLO_WORLD_BAD , tag . as_ref ()) . is_err ()) } } }
};
}
