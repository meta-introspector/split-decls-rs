// Generated macro for impl_16 (impl)
macro_rules! Depcrate_block_apiimpl_16 {
() => {
// Module: crate::block_api
// Provides: {"impl_16"}
// Dependencies: {}
impl < OS : OutputSize > FixedOutputCore for BashHashCore < OS > { fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let pos = buffer . get_pos () ; let mut block = buffer . pad_with_zeros () ; block [pos] = 0x40 ; self . compress_block (& block) ; for (src , dst) in self . state . iter () . zip (out . chunks_exact_mut (8)) { dst . copy_from_slice (& src . to_le_bytes ()) ; } } }
};
}
