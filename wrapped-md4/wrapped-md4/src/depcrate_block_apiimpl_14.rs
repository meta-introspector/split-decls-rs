// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl FixedOutputCore for Md4Core { # [inline] fn finalize_fixed_core (& mut self , buffer : & mut Buffer < Self > , out : & mut Output < Self >) { let bits_len = self . block_len . wrapping_mul (Self :: BlockSize :: U64) . wrapping_add (buffer . get_pos () as u64) . wrapping_mul (8) ; let mut state = self . state ; buffer . len64_padding_le (bits_len , | block | compress (& mut state , block . as_ref ())) ; for (chunk , v) in out . chunks_exact_mut (4) . zip (state . iter ()) { chunk . copy_from_slice (& v . to_le_bytes ()) ; } } }
};
}
