// Generated macro for impl_50 (impl)
macro_rules! Depcrate_hashimpl_50 {
() => {
// Module: crate::hash
// Provides: {"impl_50"}
// Dependencies: {}
impl hash :: Hash for Sha256 { fn start (& self) -> Box < dyn hash :: Context > { Box :: new (Sha256Context (sha2 :: Sha256 :: new ())) } fn hash (& self , data : & [u8]) -> hash :: Output { hash :: Output :: new (& sha2 :: Sha256 :: digest (data) [..]) } fn algorithm (& self) -> HashAlgorithm { HashAlgorithm :: SHA256 } fn output_len (& self) -> usize { 32 } }
};
}
