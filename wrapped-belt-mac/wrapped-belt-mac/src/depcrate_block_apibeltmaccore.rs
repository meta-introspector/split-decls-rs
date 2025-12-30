// Generated macro for BeltMacCore (struct)
macro_rules! Depcrate_block_apiBeltMacCore {
() => {
// Module: crate::block_api
// Provides: {"BeltMacCore"}
// Dependencies: {}
# [doc = " Generic core BeltMac instance, which operates over blocks."] # [derive (Clone)] pub struct BeltMacCore < C = BeltBlock > where C : BlockCipherEncrypt + Clone , { cipher : C , state : Block < C > , r : Block < C > , }
};
}
