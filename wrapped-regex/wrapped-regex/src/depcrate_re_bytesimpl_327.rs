// Generated macro for impl_327 (impl)
macro_rules! Depcrate_re_bytesimpl_327 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'a > Replacer for NoExpand < 'a > { fn replace_append (& mut self , _ : & Captures , dst : & mut Vec < u8 >) { extend_from_slice (dst , self . 0) ; } fn no_expansion < 'r > (& 'r mut self) -> Option < Cow < 'r , [u8] > > { Some (Cow :: Borrowed (self . 0)) } }
};
}
