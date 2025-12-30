// Generated macro for tests (module)
macro_rules! Depcrate_ticketertests {
() => {
// Module: crate::ticketer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use rustls :: crypto :: TicketerFactory ; use crate :: Provider ; # [test] fn basic_pairwise_test () { let t = Provider . ticketer () . unwrap () ; let cipher = t . encrypt (b"hello world") . unwrap () ; let plain = t . decrypt (& cipher) . unwrap () ; assert_eq ! (plain , b"hello world") ; } # [test] fn refuses_decrypt_before_encrypt () { let t = Provider . ticketer () . unwrap () ; assert_eq ! (t . decrypt (b"hello") , None) ; } # [test] fn refuses_decrypt_larger_than_largest_encryption () { let t = Provider . ticketer () . unwrap () ; let mut cipher = t . encrypt (b"hello world") . unwrap () ; assert_eq ! (t . decrypt (& cipher) , Some (b"hello world" . to_vec ())) ; cipher . push (0) ; assert_eq ! (t . decrypt (& cipher) , None) ; } # [test] fn aead_ticketer_is_debug_and_producestickets () { use alloc :: format ; use super :: * ; let t = AeadTicketer :: new () . unwrap () ; assert_eq ! (format ! ("{t:?}") , "AeadTicketer { .. }") ; assert_eq ! (t . lifetime () , Duration :: ZERO) ; } }
};
}
