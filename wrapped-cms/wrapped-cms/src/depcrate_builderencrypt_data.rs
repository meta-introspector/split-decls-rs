// Generated macro for encrypt_data (function)
macro_rules! Depcrate_builderencrypt_data {
() => {
// Module: crate::builder
// Provides: {"encrypt_data"}
// Dependencies: {}
# [doc = " Symmetrically encrypt data."] # [doc = " Returns encrypted content, content-encryption key and the used algorithm identifier (including"] # [doc = " the used algorithm parameters)."] # [doc = ""] # [doc = " TODO Which encryption algorithms shall also be supported?"] fn encrypt_data < R > (data : & [u8] , encryption_algorithm_identifier : & ContentEncryptionAlgorithm , key : Option < & [u8] > , rng : & mut R ,) -> Result < (Vec < u8 > , Vec < u8 > , AlgorithmIdentifierOwned) > where R : CryptoRng + ? Sized , { match encryption_algorithm_identifier { ContentEncryptionAlgorithm :: Aes128Cbc => encrypt_block_mode ! (data , cbc :: Encryptor < Aes128 >, key , rng , encryption_algorithm_identifier . oid ()) , ContentEncryptionAlgorithm :: Aes192Cbc => encrypt_block_mode ! (data , cbc :: Encryptor < Aes192 >, key , rng , encryption_algorithm_identifier . oid ()) , ContentEncryptionAlgorithm :: Aes256Cbc => encrypt_block_mode ! (data , cbc :: Encryptor < Aes256 >, key , rng , encryption_algorithm_identifier . oid ()) , } }
};
}
