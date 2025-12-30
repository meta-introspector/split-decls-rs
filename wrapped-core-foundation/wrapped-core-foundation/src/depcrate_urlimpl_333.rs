// Generated macro for impl_333 (impl)
macro_rules! Depcrate_urlimpl_333 {
() => {
// Module: crate::url
// Provides: {"impl_333"}
// Dependencies: {}
impl fmt :: Debug for CFURL { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { unsafe { let string : CFString = TCFType :: wrap_under_get_rule (CFURLGetString (self . 0)) ; write ! (f , "{string}") } } }
};
}
