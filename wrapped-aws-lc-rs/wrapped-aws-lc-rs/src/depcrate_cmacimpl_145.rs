// Generated macro for impl_145 (impl)
macro_rules! Depcrate_cmacimpl_145 {
() => {
// Module: crate::cmac
// Provides: {"impl_145"}
// Dependencies: {}
impl AlgorithmId { fn evp_cipher (& self) -> ConstPointer < '_ , EVP_CIPHER > { unsafe { ConstPointer :: new_static (match self { AlgorithmId :: Aes128 => EVP_aes_128_cbc () , AlgorithmId :: Aes192 => EVP_aes_192_cbc () , AlgorithmId :: Aes256 => EVP_aes_256_cbc () , AlgorithmId :: Tdes => EVP_des_ede3_cbc () , }) . unwrap () } } }
};
}
