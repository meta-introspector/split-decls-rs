// Generated macro for impl_8805 (impl)
macro_rules! Depcrate_ptrimpl_8805 {
() => {
// Module: crate::ptr
// Provides: {"impl_8805"}
// Dependencies: {}
impl fmt :: Display for RefPrefix { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use fmt :: Write ; f . write_char ('&') ? ; if ! self . lt . is_anonymous () { self . lt . ident . fmt (f) ? ; f . write_char (' ') ? ; } f . write_str (self . mutability . prefix_str ()) } }
};
}
