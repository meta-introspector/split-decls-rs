// Generated macro for impl_321 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_321 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_321"}
// Dependencies: {}
impl < R , Rsdr > CryptoBlockRng for ReseedingCore < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
};
}
