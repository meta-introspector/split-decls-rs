// Generated macro for tests (module)
macro_rules! Depcrate_lmstests {
() => {
// Module: crate::lms
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use :: signature :: { RandomizedSignerMut , Verifier } ; use super :: * ; use crate :: { lms :: SigningKey , ots :: LmsOtsSha256N32W4 } ; fn test_sign_and_verify < Mode : LmsMode > () { let mut rng = rand :: rng () ; let mut sk = SigningKey :: < Mode > :: new (& mut rng) ; let pk = sk . public () ; let msg = "this is a test message" . as_bytes () ; let sig = sk . try_sign_with_rng (& mut rng , msg) ; let sig = sig . unwrap () ; assert ! (pk . verify (msg , & sig) . is_ok ()) ; } # [test] fn test_sign_and_verify_lms_sha256_m32_h5_lmsots_sha256_n32_w4 () { test_sign_and_verify :: < LmsSha256M32H5 < LmsOtsSha256N32W4 > > () ; } }
};
}
