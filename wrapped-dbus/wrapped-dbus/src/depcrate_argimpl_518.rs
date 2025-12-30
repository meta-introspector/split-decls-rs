// Generated macro for impl_518 (impl)
macro_rules! Depcrate_argimpl_518 {
() => {
// Module: crate::arg
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'a > fmt :: Debug for Iter < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut z = self . clone () ; let mut t = f . debug_tuple ("Iter") ; loop { t . field (& z . arg_type ()) ; if ! z . next () { break } } t . finish () } }
};
}
