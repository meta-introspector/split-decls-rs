// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl UpdateCore for Md4Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len = self . block_len . wrapping_add (blocks . len () as u64) ; for block in blocks { compress (& mut self . state , block . as_ref ()) ; } } }
};
}
