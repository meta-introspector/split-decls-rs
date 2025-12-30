// Generated macro for impl_12 (impl)
macro_rules! Depcrate_block_apiimpl_12 {
() => {
// Module: crate::block_api
// Provides: {"impl_12"}
// Dependencies: {}
impl < C > InnerInit for CbcMacCore < C > where C : BlockCipherEncrypt + Clone , { # [inline] fn inner_init (cipher : C) -> Self { let state = Default :: default () ; Self { cipher , state } } }
};
}
