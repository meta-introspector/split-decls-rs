// Generated macro for impl_22 (impl)
macro_rules! Depcrate_block_apiimpl_22 {
() => {
// Module: crate::block_api
// Provides: {"impl_22"}
// Dependencies: {}
impl < C : PmacCipher , const LC_SIZE : usize > FixedOutputCore for PmacCore < C , LC_SIZE > { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let Self { cipher , state : PmacState { tag , l_inv , .. } , } = self ; let pos = buffer . get_pos () ; let buf = buffer . pad_with_zeros () ; if pos == buf . len () { xor (tag , & buf) ; xor (tag , l_inv) ; } else { tag [pos] ^= 0x80 ; xor (tag , & buf) ; } cipher . encrypt_block_b2b (tag , out) ; } }
};
}
