// Generated macro for impl_39 (impl)
macro_rules! Depcrate_simpleimpl_39 {
() => {
// Module: crate::simple
// Provides: {"impl_39"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > Update for SimpleHmac < D > { # [inline (always)] fn update (& mut self , data : & [u8]) { self . digest . update (data) ; } }
};
}
