// Generated macro for impl_95 (impl)
macro_rules! Depcrate_errorimpl_95 {
() => {
// Module: crate::error
// Provides: {"impl_95"}
// Dependencies: {}
impl Display for MissingPrefixBufError { # [inline] fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "path is missing a prefix: \"{}\"" , self . 0 . display ()) } }
};
}
