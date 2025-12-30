// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl UpdateCore for Md2Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { self . compress (block . as_ref ()) } } }
};
}
