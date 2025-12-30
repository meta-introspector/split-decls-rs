// Generated macro for tests (module)
macro_rules! Depcrate_ecdsatests {
() => {
// Module: crate::ecdsa
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , feature = "ecdsa"))] mod tests { mod verify { use crate :: { NistP192 , test_vectors :: ecdsa :: ECDSA_TEST_VECTORS } ; ecdsa_core :: new_verification_test ! (NistP192 , ECDSA_TEST_VECTORS) ; } }
};
}
