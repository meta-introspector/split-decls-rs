// Generated macro for forward_fmt_impl (macro)
macro_rules! Depcrate_rc_retained_forwarding_implsforward_fmt_impl {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"forward_fmt_impl"}
// Dependencies: {}
macro_rules ! forward_fmt_impl { ($ trait : path) => { impl < T : $ trait + ? Sized > $ trait for Retained < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { (** self) . fmt (f) } } } ; }
};
}
