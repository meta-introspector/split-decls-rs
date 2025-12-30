// Generated macro for impl_10 (impl)
macro_rules! Depcrate_block_apiimpl_10 {
() => {
// Module: crate::block_api
// Provides: {"impl_10"}
// Dependencies: {}
impl < C > KeySizeUser for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , < C as BlockSizeUser > :: BlockSize : Mul < U2 > , Prod < < C as BlockSizeUser > :: BlockSize , U2 > : ArraySize , { type KeySize = Prod < < C as BlockSizeUser > :: BlockSize , U2 > ; }
};
}
