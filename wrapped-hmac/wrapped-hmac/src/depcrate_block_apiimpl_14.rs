// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl < D : EagerHash > UpdateCore for HmacCore < D > { # [inline (always)] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . digest . update_blocks (blocks) ; } }
};
}
