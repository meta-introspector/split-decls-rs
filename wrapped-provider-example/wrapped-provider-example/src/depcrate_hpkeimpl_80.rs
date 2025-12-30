// Generated macro for impl_80 (impl)
macro_rules! Depcrate_hpkeimpl_80 {
() => {
// Module: crate::hpke
// Provides: {"impl_80"}
// Dependencies: {}
impl HpkeRs { fn start (& self) -> Result < hpke_rs :: Hpke < HpkeRustCrypto > , Error > { Ok (hpke_rs :: Hpke :: new (hpke_rs :: Mode :: Base , KemAlgorithm :: try_from (u16 :: from (self . 0 . kem)) . map_err (other_err) ? , KdfAlgorithm :: try_from (u16 :: from (self . 0 . sym . kdf_id)) . map_err (other_err) ? , AeadAlgorithm :: try_from (u16 :: from (self . 0 . sym . aead_id)) . map_err (other_err) ? ,)) } }
};
}
