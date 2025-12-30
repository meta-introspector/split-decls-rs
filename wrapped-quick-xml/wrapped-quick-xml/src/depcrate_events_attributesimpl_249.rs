// Generated macro for impl_249 (impl)
macro_rules! Depcrate_events_attributesimpl_249 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_249"}
// Dependencies: {}
impl < 'a > Debug for Attribute < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . debug_struct ("Attribute") . field ("key" , & Bytes (self . key . as_ref ())) . field ("value" , & Bytes (& self . value)) . finish () } }
};
}
