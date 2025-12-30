// Generated macro for impl_102 (impl)
macro_rules! Depcrate_ext_deimpl_102 {
() => {
// Module: crate::ext::de
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'de > ExtDeserializer < 'de > { const fn new_owned (tag : i8 , data : Vec < u8 >) -> Self { ExtDeserializer { tag : Some (tag) , data : Some (Cow :: Owned (data)) , } } const fn new_ref (tag : i8 , data : & 'de [u8]) -> Self { ExtDeserializer { tag : Some (tag) , data : Some (Cow :: Borrowed (data)) , } } }
};
}
