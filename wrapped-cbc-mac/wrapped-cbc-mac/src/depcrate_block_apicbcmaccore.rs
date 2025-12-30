// Generated macro for CbcMacCore (struct)
macro_rules! Depcrate_block_apiCbcMacCore {
() => {
// Module: crate::block_api
// Provides: {"CbcMacCore"}
// Dependencies: {}
# [doc = " Generic core CMAC instance, which operates over blocks."] # [derive (Clone)] pub struct CbcMacCore < C > where C : BlockCipherEncrypt + Clone , { cipher : C , state : Block < C > , }
};
}
