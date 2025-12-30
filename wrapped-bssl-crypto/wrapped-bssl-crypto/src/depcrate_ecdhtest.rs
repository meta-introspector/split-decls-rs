// Generated macro for test (module)
macro_rules! Depcrate_ecdhtest {
() => {
// Module: crate::ecdh
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: ec :: { P256 , P384 } ; fn check_curve < C : ec :: Curve > () { let alice_private_key = PrivateKey :: < C > :: generate () ; let alice_public_key = alice_private_key . to_public_key () ; let alice_private_key = PrivateKey :: < C > :: from_big_endian (alice_private_key . to_big_endian () . as_ref ()) . unwrap () ; let alice_private_key = PrivateKey :: < C > :: from_der_ec_private_key (alice_private_key . to_der_ec_private_key () . as_ref () ,) . unwrap () ; let bob_private_key = PrivateKey :: < C > :: generate () ; let bob_public_key = bob_private_key . to_public_key () ; let shared_key1 = alice_private_key . compute_shared_key (& bob_public_key) ; let shared_key2 = bob_private_key . compute_shared_key (& alice_public_key) ; assert_eq ! (shared_key1 , shared_key2) ; } # [test] fn p256 () { check_curve :: < P256 > () ; } # [test] fn p384 () { check_curve :: < P384 > () ; } }
};
}
