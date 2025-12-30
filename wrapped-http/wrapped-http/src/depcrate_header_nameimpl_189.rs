// Generated macro for impl_189 (impl)
macro_rules! Depcrate_header_nameimpl_189 {
() => {
// Module: crate::header::name
// Provides: {"impl_189"}
// Dependencies: {}
impl < 'a > Hash for MaybeLower < 'a > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { if self . lower { hasher . write (self . buf) ; } else { for & b in self . buf { hasher . write (& [HEADER_CHARS [b as usize]]) ; } } } }
};
}
