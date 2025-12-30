// Generated macro for impl_152 (impl)
macro_rules! Depcrate_fallbackimpl_152 {
() => {
// Module: crate::fallback
// Provides: {"impl_152"}
// Dependencies: {}
impl Debug for Group { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = fmt . debug_struct ("Group") ; debug . field ("delimiter" , & self . delimiter) ; debug . field ("stream" , & self . stream) ; debug_span_field_if_nontrivial (& mut debug , self . span) ; debug . finish () } }
};
}
