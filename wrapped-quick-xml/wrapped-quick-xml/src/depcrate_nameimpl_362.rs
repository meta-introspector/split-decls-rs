// Generated macro for impl_362 (impl)
macro_rules! Depcrate_nameimpl_362 {
() => {
// Module: crate::name
// Provides: {"impl_362"}
// Dependencies: {}
impl < 'ns > Debug for ResolveResult < 'ns > { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { Self :: Unbound => write ! (f , "Unbound") , Self :: Bound (ns) => write ! (f , "Bound({:?})" , ns) , Self :: Unknown (p) => { write ! (f , "Unknown(") ? ; write_byte_string (f , p) ? ; write ! (f , ")") } } } }
};
}
