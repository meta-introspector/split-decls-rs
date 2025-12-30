// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl Generate for DeleteAccountAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { beneficiary_id : AccountId :: generate (rng) , } } }
};
}
