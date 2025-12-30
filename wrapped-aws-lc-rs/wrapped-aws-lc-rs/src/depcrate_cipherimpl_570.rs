// Generated macro for impl_570 (impl)
macro_rules! Depcrate_cipherimpl_570 {
() => {
// Module: crate::cipher
// Provides: {"impl_570"}
// Dependencies: {}
impl OperatingMode { fn evp_cipher (& self , algorithm : & Algorithm) -> ConstPointer < '_ , EVP_CIPHER > { unsafe { ConstPointer :: new_static (match (self , algorithm . id) { (OperatingMode :: CBC , AlgorithmId :: Aes128) => EVP_aes_128_cbc () , (OperatingMode :: CTR , AlgorithmId :: Aes128) => EVP_aes_128_ctr () , (OperatingMode :: CFB128 , AlgorithmId :: Aes128) => EVP_aes_128_cfb128 () , (OperatingMode :: ECB , AlgorithmId :: Aes128) => EVP_aes_128_ecb () , (OperatingMode :: CBC , AlgorithmId :: Aes192) => EVP_aes_192_cbc () , (OperatingMode :: CTR , AlgorithmId :: Aes192) => EVP_aes_192_ctr () , (OperatingMode :: CFB128 , AlgorithmId :: Aes192) => EVP_aes_192_cfb128 () , (OperatingMode :: ECB , AlgorithmId :: Aes192) => EVP_aes_192_ecb () , (OperatingMode :: CBC , AlgorithmId :: Aes256) => EVP_aes_256_cbc () , (OperatingMode :: CTR , AlgorithmId :: Aes256) => EVP_aes_256_ctr () , (OperatingMode :: CFB128 , AlgorithmId :: Aes256) => EVP_aes_256_cfb128 () , (OperatingMode :: ECB , AlgorithmId :: Aes256) => EVP_aes_256_ecb () , }) . unwrap () } } }
};
}
