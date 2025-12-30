// Generated macro for impl_52 (impl)
macro_rules! Depcrate_simple_resetimpl_52 {
() => {
// Module: crate::simple_reset
// Provides: {"impl_52"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > Update for SimpleHmacReset < D > { # [inline (always)] fn update (& mut self , data : & [u8]) { self . digest . update (data) ; } }
};
}
