// Generated macro for impl_283 (impl)
macro_rules! Depcrateimpl_283 {
() => {
// Module: crate
// Provides: {"impl_283"}
// Dependencies: {}
impl Debug for Punct { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let mut debug = fmt . debug_struct ("Punct") ; debug . field ("char" , & self . ch) ; debug . field ("spacing" , & self . spacing) ; imp :: debug_span_field_if_nontrivial (& mut debug , self . span . inner) ; debug . finish () } }
};
}
