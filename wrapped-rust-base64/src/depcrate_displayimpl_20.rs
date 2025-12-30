// Generated macro for impl_20 (impl)
macro_rules! Depcrate_displayimpl_20 {
() => {
// Module: crate::display
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , 'e , E : Engine > Base64Display < 'a , 'e , E > { # [doc = " Create a `Base64Display` with the provided engine."] pub fn new (bytes : & 'a [u8] , engine : & 'e E) -> Base64Display < 'a , 'e , E > { Base64Display { bytes , chunked_encoder : ChunkedEncoder :: new (engine) , } } }
};
}
