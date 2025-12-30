// Generated macro for impl_271 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_271 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_271"}
// Dependencies: {}
impl < R , Rsdr > CryptoRng for ReseedingRng < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
};
}
