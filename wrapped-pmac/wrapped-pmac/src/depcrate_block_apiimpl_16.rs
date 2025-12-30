// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > Reset for PmacCore < C , LC_SIZE > { # [inline (always)] fn reset (& mut self) { self . state . tag = Default :: default () ; self . state . offset = Default :: default () ; self . state . counter = 1 ; } }
};
}
