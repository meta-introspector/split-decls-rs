// Generated macro for impl_13 (impl)
macro_rules! Depcrate_block_apiimpl_13 {
() => {
// Module: crate::block_api
// Provides: {"impl_13"}
// Dependencies: {}
impl UpdateCore for BeltHashCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . r = self . r . wrapping_add (blocks . len () as u128) ; for block in blocks { self . compress_block (block) ; } } }
};
}
