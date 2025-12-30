// Generated macro for impl_84 (impl)
macro_rules! Depcrate_numimpl_84 {
() => {
// Module: crate::num
// Provides: {"impl_84"}
// Dependencies: {}
impl < F : FloatExt > Consts < F > { fn new () -> Self { let top_sigbit_mask = F :: Int :: ONE << (F :: SIG_BITS - 1) ; let pos_nan = F :: EXP_MASK | top_sigbit_mask ; let max_qnan = F :: EXP_MASK | F :: SIG_MASK ; let min_snan = F :: EXP_MASK | F :: Int :: ONE ; let max_snan = (F :: EXP_MASK | F :: SIG_MASK) ^ top_sigbit_mask ; let neg_nan = pos_nan | F :: SIGN_MASK ; let neg_max_qnan = max_qnan | F :: SIGN_MASK ; let neg_min_snan = min_snan | F :: SIGN_MASK ; let neg_max_snan = max_snan | F :: SIGN_MASK ; Self { pos_nan : F :: from_bits (pos_nan) , neg_nan : F :: from_bits (neg_nan) , max_qnan : F :: from_bits (max_qnan) , min_snan : F :: from_bits (min_snan) , max_snan : F :: from_bits (max_snan) , neg_max_qnan : F :: from_bits (neg_max_qnan) , neg_min_snan : F :: from_bits (neg_min_snan) , neg_max_snan : F :: from_bits (neg_max_snan) , } } pub fn iter (self) -> impl Iterator < Item = F > { let Self { pos_nan , neg_nan , max_qnan , min_snan , max_snan , neg_max_qnan , neg_min_snan , neg_max_snan , } = self ; [pos_nan , neg_nan , max_qnan , min_snan , max_snan , neg_max_qnan , neg_min_snan , neg_max_snan ,] . into_iter () } }
};
}
