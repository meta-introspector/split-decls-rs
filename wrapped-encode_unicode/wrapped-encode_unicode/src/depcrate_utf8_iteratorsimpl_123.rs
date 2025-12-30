// Generated macro for impl_123 (impl)
macro_rules! Depcrate_utf8_iteratorsimpl_123 {
() => {
// Module: crate::utf8_iterators
// Provides: {"impl_123"}
// Dependencies: {}
impl fmt :: Debug for Utf8Iterator { fn fmt (& self , fmtr : & mut fmt :: Formatter) -> fmt :: Result { let mut content = [0 ; 4] ; let mut i = 0 ; for b in self . clone () { content [i] = b ; i += 1 ; } write ! (fmtr , "{:?}" , & content [.. i]) } }
};
}
