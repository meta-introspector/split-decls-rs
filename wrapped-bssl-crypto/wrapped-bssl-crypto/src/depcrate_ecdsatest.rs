// Generated macro for test (module)
macro_rules! Depcrate_ecdsatest {
() => {
// Module: crate::ecdsa
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: ec :: { P256 , P384 } ; fn check_curve < C : ec :: Curve > () { let signed_message = b"hello world" ; let key = PrivateKey :: < C > :: generate () ; let mut sig = key . sign (signed_message) ; let mut sig_p1363 = key . sign_p1363 (signed_message) ; let public_key = PublicKey :: < C > :: from_der_subject_public_key_info (key . to_der_subject_public_key_info () . as_ref () ,) . unwrap () ; assert ! (public_key . verify (signed_message , sig . as_slice ()) . is_ok ()) ; assert ! (public_key . verify_p1363 (signed_message , sig_p1363 . as_slice ()) . is_ok ()) ; sig [10] ^= 1 ; assert ! (public_key . verify (signed_message , sig . as_slice ()) . is_err ()) ; sig_p1363 [10] ^= 1 ; assert ! (public_key . verify_p1363 (signed_message , sig_p1363 . as_slice ()) . is_err ()) ; } # [test] fn p256 () { check_curve :: < P256 > () ; } # [test] fn p384 () { check_curve :: < P384 > () ; } }
};
}
