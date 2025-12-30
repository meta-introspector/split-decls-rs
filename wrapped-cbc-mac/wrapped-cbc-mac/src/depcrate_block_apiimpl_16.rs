// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < C > FixedOutputCore for CbcMacCore < C > where C : BlockCipherEncrypt + Clone , { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let Self { state , cipher } = self ; let pos = buffer . get_pos () ; if pos != 0 { xor (state , & buffer . pad_with_zeros ()) ; cipher . encrypt_block (state) ; } out . copy_from_slice (state) ; } }
};
}
