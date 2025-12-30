// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl < C > InnerInit for BeltMacCore < C > where C : BlockCipherEncrypt + Clone , { # [inline] fn inner_init (cipher : C) -> Self { let state = Default :: default () ; let mut r = Default :: default () ; cipher . encrypt_block (& mut r) ; Self { cipher , state , r } } }
};
}
