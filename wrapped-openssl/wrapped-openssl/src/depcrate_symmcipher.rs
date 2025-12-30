// Generated macro for Cipher (struct)
macro_rules! Depcrate_symmCipher {
() => {
// Module: crate::symm
// Provides: {"Cipher"}
// Dependencies: {}
# [doc = " Represents a particular cipher algorithm."] # [doc = ""] # [doc = " See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms."] # [doc = ""] # [doc = " [`EVP_EncryptInit`]: https://docs.openssl.org/master/man3/EVP_EncryptInit/"] # [derive (Copy , Clone , PartialEq , Eq)] pub struct Cipher (* const ffi :: EVP_CIPHER) ;
};
}
