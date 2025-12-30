// Generated macro for impl_273 (impl)
macro_rules! Depcrate_rngs_reseedingimpl_273 {
() => {
// Module: crate::rngs::reseeding
// Provides: {"impl_273"}
// Dependencies: {}
impl < R , Rsdr > BlockRngCore for ReseedingCore < R , Rsdr > where R : BlockRngCore + SeedableRng , Rsdr : TryRngCore , { type Item = < R as BlockRngCore > :: Item ; type Results = < R as BlockRngCore > :: Results ; fn generate (& mut self , results : & mut Self :: Results) { if self . bytes_until_reseed <= 0 { return self . reseed_and_generate (results) ; } let num_bytes = size_of_val (results . as_ref ()) ; self . bytes_until_reseed -= num_bytes as i64 ; self . inner . generate (results) ; } }
};
}
