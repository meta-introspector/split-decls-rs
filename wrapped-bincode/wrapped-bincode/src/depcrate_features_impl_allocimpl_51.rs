// Generated macro for impl_51 (impl)
macro_rules! Depcrate_features_impl_allocimpl_51 {
() => {
// Module: crate::features::impl_alloc
// Provides: {"impl_51"}
// Dependencies: {}
impl enc :: write :: Writer for VecWriter { # [inline (always)] fn write (& mut self , bytes : & [u8]) -> Result < () , EncodeError > { self . inner . extend_from_slice (bytes) ; Ok (()) } }
};
}
