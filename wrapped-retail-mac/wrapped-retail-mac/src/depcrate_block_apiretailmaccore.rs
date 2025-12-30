// Generated macro for RetailMacCore (struct)
macro_rules! Depcrate_block_apiRetailMacCore {
() => {
// Module: crate::block_api
// Provides: {"RetailMacCore"}
// Dependencies: {}
# [doc = " Generic core Retail MAC instance, which operates over blocks."] # [derive (Clone)] pub struct RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , { cipher : C , cipher_prime : C , state : Block < C > , }
};
}
