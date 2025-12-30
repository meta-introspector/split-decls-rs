// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Display for GlobError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "attempting to read `{}` resulted in an error: {}" , self . path . display () , self . error) } }
};
}
