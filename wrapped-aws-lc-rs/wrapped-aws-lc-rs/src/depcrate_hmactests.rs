// Generated macro for tests (module)
macro_rules! Depcrate_hmactests {
() => {
// Module: crate::hmac
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { hmac , rand } ; # [cfg (feature = "fips")] mod fips ; # [test] pub fn hmac_signing_key_coverage () { const HELLO_WORLD_GOOD : & [u8] = b"hello, world" ; const HELLO_WORLD_BAD : & [u8] = b"hello, worle" ; let rng = rand :: SystemRandom :: new () ; for algorithm in & [hmac :: HMAC_SHA1_FOR_LEGACY_USE_ONLY , hmac :: HMAC_SHA224 , hmac :: HMAC_SHA256 , hmac :: HMAC_SHA384 , hmac :: HMAC_SHA512 ,] { let key = hmac :: Key :: generate (* algorithm , & rng) . unwrap () ; let tag = hmac :: sign (& key , HELLO_WORLD_GOOD) ; println ! ("{key:?}") ; assert ! (hmac :: verify (& key , HELLO_WORLD_GOOD , tag . as_ref ()) . is_ok ()) ; assert ! (hmac :: verify (& key , HELLO_WORLD_BAD , tag . as_ref ()) . is_err ()) ; } } # [test] fn hmac_coverage () { assert_ne ! (hmac :: HMAC_SHA256 , hmac :: HMAC_SHA384) ; for & alg in & [hmac :: HMAC_SHA1_FOR_LEGACY_USE_ONLY , hmac :: HMAC_SHA224 , hmac :: HMAC_SHA256 , hmac :: HMAC_SHA384 , hmac :: HMAC_SHA512 ,] { let key = hmac :: Key :: new (alg , & [0 ; 32]) ; let mut ctx = hmac :: Context :: with_key (& key) ; ctx . update (b"hello, world") ; let ctx_clone = ctx . clone () ; let orig_tag = ctx . sign () ; let clone_tag = ctx_clone . sign () ; assert_eq ! (orig_tag . as_ref () , clone_tag . as_ref ()) ; assert_eq ! (orig_tag . clone () . as_ref () , clone_tag . as_ref ()) ; } } }
};
}
