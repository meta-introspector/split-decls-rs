// Generated macro for impl_317 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_317 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_317"}
// Dependencies: {}
impl < R , Rsdr > CryptoRng for ReseedingRng < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
};
}
