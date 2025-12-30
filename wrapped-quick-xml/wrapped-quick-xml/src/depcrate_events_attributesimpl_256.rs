// Generated macro for impl_256 (impl)
macro_rules! Depcrate_events_attributesimpl_256 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_256"}
// Dependencies: {}
impl < 'a > Debug for Attributes < 'a > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . debug_struct ("Attributes") . field ("bytes" , & Bytes (self . bytes)) . field ("state" , & self . state) . field ("decoder" , & self . decoder) . finish () } }
};
}
