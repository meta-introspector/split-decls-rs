// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl UpdateCore for Md5Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len = self . block_len . wrapping_add (blocks . len () as u64) ; let blocks = Array :: cast_slice_to_core (blocks) ; compress (& mut self . state , blocks) } }
};
}
