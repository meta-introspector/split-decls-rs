// Generated macro for impl_153 (impl)
macro_rules! Depcrate_encodeimpl_153 {
() => {
// Module: crate::encode
// Provides: {"impl_153"}
// Dependencies: {}
impl < W : Write , C > UnderlyingWrite for Serializer < W , C > { type Write = W ; # [inline (always)] fn get_ref (& self) -> & Self :: Write { & self . wr } # [inline (always)] fn get_mut (& mut self) -> & mut Self :: Write { & mut self . wr } # [inline (always)] fn into_inner (self) -> Self :: Write { self . wr } }
};
}
