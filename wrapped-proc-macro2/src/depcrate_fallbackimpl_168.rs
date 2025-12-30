// Generated macro for impl_168 (impl)
macro_rules! Depcrate_fallbackimpl_168 {
() => {
// Module: crate::fallback
// Provides: {"impl_168"}
// Dependencies: {}
impl Debug for Literal { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = fmt . debug_struct ("Literal") ; debug . field ("lit" , & format_args ! ("{}" , self . repr)) ; debug_span_field_if_nontrivial (& mut debug , self . span) ; debug . finish () } }
};
}
