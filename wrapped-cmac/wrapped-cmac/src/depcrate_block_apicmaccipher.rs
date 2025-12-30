// Generated macro for CmacCipher (trait)
macro_rules! Depcrate_block_apiCmacCipher {
() => {
// Module: crate::block_api
// Provides: {"CmacCipher"}
// Dependencies: {}
# [doc = " Helper trait implemented for cipher supported by CMAC"] pub trait CmacCipher : BlockSizeUser + BlockCipherEncrypt + Clone { # [doc = " Double block. See the [`Dbl`] trait docs for more information."] fn dbl (block : Block < Self >) -> Block < Self > ; }
};
}
