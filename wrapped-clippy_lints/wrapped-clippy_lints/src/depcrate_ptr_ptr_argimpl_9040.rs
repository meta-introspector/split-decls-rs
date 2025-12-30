// Generated macro for impl_9040 (impl)
macro_rules! Depcrate_ptr_ptr_argimpl_9040 {
() => {
// Module: crate::ptr::ptr_arg
// Provides: {"impl_9040"}
// Dependencies: {}
impl fmt :: Display for RefPrefix { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use fmt :: Write ; f . write_char ('&') ? ; if ! self . lt . is_anonymous () { self . lt . ident . fmt (f) ? ; f . write_char (' ') ? ; } f . write_str (self . mutability . prefix_str ()) } }
};
}
