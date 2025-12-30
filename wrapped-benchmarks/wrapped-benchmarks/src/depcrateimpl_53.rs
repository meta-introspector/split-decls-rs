// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl Generate for DeployContractAction { fn generate < R : rand :: Rng > (rng : & mut R) -> Self { Self { code : generate_vec_u8 (rng , 20 * 2usize . pow (10) , 2usize . pow (20)) , } } }
};
}
