// Generated macro for tests (module)
macro_rules! Depcrate_devtests {
() => {
// Module: crate::dev
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; impl crate :: hazmat :: DigestAlgorithm for MockCurve { type Digest = sha2 :: Sha256 ; } new_wycheproof_test ! (wycheproof_mock , "wycheproof-mock" , MockCurve) ; }
};
}
