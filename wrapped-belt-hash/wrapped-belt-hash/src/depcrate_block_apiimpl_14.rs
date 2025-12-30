// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl FixedOutputCore for BeltHashCore { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; if pos != 0 { let block = buffer . pad_with_zeros () ; self . compress_block (& block) ; } let bs = Self :: BlockSize :: USIZE as u128 ; let r = encode_r (8 * ((bs * self . r) + pos as u128)) ; let (_ , y) = belt_compress (r , self . s , self . h) ; write_u32s (& y , out) ; } }
};
}
