// Generated macro for impl_24 (impl)
macro_rules! Depcrate_literalsimpl_24 {
() => {
// Module: crate::literals
// Provides: {"impl_24"}
// Dependencies: {}
impl fmt :: Debug for Lit { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . is_cut () { write ! (f , "Cut({})" , escape_unicode (& self . v)) } else { write ! (f , "Complete({})" , escape_unicode (& self . v)) } } }
};
}
