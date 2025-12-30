// Generated macro for encrypt (function)
macro_rules! Depcrate_cipherencrypt {
() => {
// Module: crate::cipher
// Provides: {"encrypt"}
// Dependencies: {}
fn encrypt (algorithm : & Algorithm , key : & SymmetricCipherKey , mode : OperatingMode , in_out : & mut [u8] , context : EncryptionContext ,) -> Result < DecryptionContext , Unspecified > { let block_len = algorithm . block_len () ; match mode { OperatingMode :: CBC | OperatingMode :: ECB => { if in_out . len () % block_len != 0 { return Err (Unspecified) ; } } _ => { } } match mode { OperatingMode :: CBC => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: encrypt_cbc_mode (key , context , in_out) } } , OperatingMode :: CTR => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: encrypt_ctr_mode (key , context , in_out) } } , OperatingMode :: CFB128 => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: encrypt_cfb_mode (key , mode , context , in_out) } } , OperatingMode :: ECB => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: encrypt_ecb_mode (key , context , in_out) } } , } }
};
}
