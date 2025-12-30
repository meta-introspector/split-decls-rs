// Generated macro for impl_10 (impl)
macro_rules! Depcrate_block_apiimpl_10 {
() => {
// Module: crate::block_api
// Provides: {"impl_10"}
// Dependencies: {}
impl < OS : OutputSize > BashHashCore < OS > { # [doc = " Compress one data block"] fn compress_block (& mut self , block : & Block < Self >) { for (dst , chunk) in self . state . iter_mut () . zip (block . chunks_exact (8)) { * dst = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } bash_f (& mut self . state) ; } }
};
}
