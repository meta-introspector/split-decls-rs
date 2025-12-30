// Generated macro for decrypt (function)
macro_rules! Depcrate_cipherdecrypt {
() => {
// Module: crate::cipher
// Provides: {"decrypt"}
// Dependencies: {}
fn decrypt < 'in_out > (algorithm : & 'static Algorithm , key : & SymmetricCipherKey , mode : OperatingMode , in_out : & 'in_out mut [u8] , context : DecryptionContext ,) -> Result < & 'in_out mut [u8] , Unspecified > { let block_len = algorithm . block_len () ; match mode { OperatingMode :: CBC | OperatingMode :: ECB => { if in_out . len () % block_len != 0 { return Err (Unspecified) ; } } _ => { } } match mode { OperatingMode :: CBC => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: decrypt_cbc_mode (key , context , in_out) } } , OperatingMode :: CTR => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: decrypt_ctr_mode (key , context , in_out) } } , OperatingMode :: CFB128 => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: decrypt_cfb_mode (key , mode , context , in_out) } } , OperatingMode :: ECB => match algorithm . id () { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => { aes :: decrypt_ecb_mode (key , context , in_out) } } , } }
};
}
