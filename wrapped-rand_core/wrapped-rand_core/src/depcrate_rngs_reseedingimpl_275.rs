// Generated macro for impl_275 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_275 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_275"}
// Dependencies: {}
impl < R , Rsdr > CryptoBlockRng for ReseedingCore < R , Rsdr > where R : BlockRngCore < Item = u32 > + SeedableRng + CryptoBlockRng , Rsdr : TryCryptoRng , { }
};
}
