// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < C > FixedOutputCore for RetailMacCore < C > where C : BlockCipherEncrypt + BlockCipherDecrypt + Clone , { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let Self { state , cipher , cipher_prime , } = self ; let pos = buffer . get_pos () ; if pos != 0 { xor (state , & buffer . pad_with_zeros ()) ; cipher . encrypt_block (state) ; } cipher_prime . decrypt_block (state) ; cipher . encrypt_block (state) ; out . copy_from_slice (state) ; } }
};
}
