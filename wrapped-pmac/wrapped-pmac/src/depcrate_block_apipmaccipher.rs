// Generated macro for PmacCipher (trait)
macro_rules! Depcrate_block_apiPmacCipher {
() => {
// Module: crate::block_api
// Provides: {"PmacCipher"}
// Dependencies: {}
# [doc = " Helper trait implemented for block ciphers supported by PMAC."] # [doc = ""] # [doc = " Currently this trait is implemented for all block cipher encryptors"] # [doc = " with block size equal to 64 and 128 bits."] pub trait PmacCipher : BlockSizeUser + BlockCipherEncrypt + Clone { # [doc = " Double block. See the [`Dbl`] trait docs for more information."] fn dbl (block : Block < Self >) -> Block < Self > ; # [doc = " Reverse double block.. See the [`Dbl`] trait docs for more information."] fn inv_dbl (block : Block < Self >) -> Block < Self > ; }
};
}
