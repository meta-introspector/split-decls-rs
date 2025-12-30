// Generated macro for impl_26 (impl)
macro_rules! Depcrate_block_apiimpl_26 {
() => {
// Module: crate::block_api
// Provides: {"impl_26"}
// Dependencies: {}
impl < D : EagerHash > UpdateCore for HmacResetCore < D > { # [inline (always)] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . digest . update_blocks (blocks) ; } }
};
}
