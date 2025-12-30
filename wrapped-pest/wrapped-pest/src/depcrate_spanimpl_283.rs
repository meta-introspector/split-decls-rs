// Generated macro for impl_283 (impl)
macro_rules! Depcrate_spanimpl_283 {
() => {
// Module: crate::span
// Provides: {"impl_283"}
// Dependencies: {}
impl fmt :: Debug for Span < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("str" , & self . as_str ()) . field ("start" , & self . start) . field ("end" , & self . end) . finish () } }
};
}
