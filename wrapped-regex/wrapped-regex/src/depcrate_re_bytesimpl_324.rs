// Generated macro for impl_324 (impl)
macro_rules! Depcrate_re_bytesimpl_324 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_324"}
// Dependencies: {}
impl < 'a > Replacer for & 'a [u8] { fn replace_append (& mut self , caps : & Captures , dst : & mut Vec < u8 >) { caps . expand (* self , dst) ; } fn no_expansion < 'r > (& 'r mut self) -> Option < Cow < 'r , [u8] > > { match memchr (b'$' , * self) { Some (_) => None , None => Some (Cow :: Borrowed (* self)) , } } }
};
}
