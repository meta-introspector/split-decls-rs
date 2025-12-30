// Generated macro for CmacCore (struct)
macro_rules! Depcrate_block_apiCmacCore {
() => {
// Module: crate::block_api
// Provides: {"CmacCore"}
// Dependencies: {}
# [doc = " Generic core CMAC instance, which operates over blocks."] # [derive (Clone)] pub struct CmacCore < C : CmacCipher > { cipher : C , state : Block < C > , }
};
}
