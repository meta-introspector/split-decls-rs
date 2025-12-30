// Generated macro for impl_15 (impl)
macro_rules! Depcrate_block_apiimpl_15 {
() => {
// Module: crate::block_api
// Provides: {"impl_15"}
// Dependencies: {}
impl < OS : OutputSize > UpdateCore for BashHashCore < OS > { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { self . compress_block (block) ; } } }
};
}
