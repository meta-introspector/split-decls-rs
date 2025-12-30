// Generated macro for tests (module)
macro_rules! Depcrate_montgomerytests {
() => {
// Module: crate::montgomery
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_montgomery_edwards () { let scalar = EdwardsScalar :: from (200u32) ; use crate :: GOLDILOCKS_BASE_POINT as bp ; let montgomery_bp = bp . to_montgomery () ; let montgomery_res = & montgomery_bp * & scalar ; let goldilocks_point = bp . scalar_mul (& scalar) ; assert_eq ! (goldilocks_point . to_montgomery () , montgomery_res) ; } }
};
}
