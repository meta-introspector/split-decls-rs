// Generated macro for impl_26 (impl)
macro_rules! Depcrate_block_apiimpl_26 {
() => {
// Module: crate::block_api
// Provides: {"impl_26"}
// Dependencies: {}
impl < C > PmacCipher for C where Self : BlockSizeUser + BlockCipherEncrypt + Clone , Block < Self > : Dbl , { fn dbl (block : Block < Self >) -> Block < Self > { block . dbl () } fn inv_dbl (block : Block < Self >) -> Block < Self > { block . inv_dbl () } }
};
}
