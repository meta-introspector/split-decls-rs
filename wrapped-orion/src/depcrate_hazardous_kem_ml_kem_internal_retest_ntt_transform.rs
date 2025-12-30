// Generated macro for test_ntt_transform (module)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_retest_ntt_transform {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::re
// Provides: {"test_ntt_transform"}
// Dependencies: {}
# [cfg (all (test , feature = "safe_api"))] mod test_ntt_transform { use super :: * ; # [test] fn test_to_from_ntt_roundtrips () { for _ in 0 .. 100 { let f : RingElement = RingElement :: random_element () ; let f_hat : RingElementNTT = to_ntt (& RingElement :: random_element ()) ; assert_eq ! (f , inverse_ntt (& to_ntt (& f)) ,) ; assert_eq ! (f_hat , to_ntt (& inverse_ntt (& f_hat)) ,) ; } } }
};
}
