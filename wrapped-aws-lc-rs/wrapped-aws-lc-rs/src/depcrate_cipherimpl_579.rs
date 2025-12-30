// Generated macro for impl_579 (impl)
macro_rules! Depcrate_cipherimpl_579 {
() => {
// Module: crate::cipher
// Provides: {"impl_579"}
// Dependencies: {}
impl Algorithm { fn id (& self) -> & AlgorithmId { & self . id } # [doc = " The block length of this cipher algorithm."] # [must_use] pub const fn block_len (& self) -> usize { self . block_len } fn new_encryption_context (& self , mode : OperatingMode ,) -> Result < EncryptionContext , Unspecified > { match self . id { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => match mode { OperatingMode :: CBC | OperatingMode :: CTR | OperatingMode :: CFB128 => { Ok (EncryptionContext :: Iv128 (FixedLength :: new () ?)) } OperatingMode :: ECB => Ok (EncryptionContext :: None) , } , } } fn is_valid_encryption_context (& self , mode : OperatingMode , input : & EncryptionContext) -> bool { match self . id { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => match mode { OperatingMode :: CBC | OperatingMode :: CTR | OperatingMode :: CFB128 => { matches ! (input , EncryptionContext :: Iv128 (_)) } OperatingMode :: ECB => { matches ! (input , EncryptionContext :: None) } } , } } fn is_valid_decryption_context (& self , mode : OperatingMode , input : & DecryptionContext) -> bool { match self . id { AlgorithmId :: Aes128 | AlgorithmId :: Aes192 | AlgorithmId :: Aes256 => match mode { OperatingMode :: CBC | OperatingMode :: CTR | OperatingMode :: CFB128 => { matches ! (input , DecryptionContext :: Iv128 (_)) } OperatingMode :: ECB => { matches ! (input , DecryptionContext :: None) } } , } } }
};
}
