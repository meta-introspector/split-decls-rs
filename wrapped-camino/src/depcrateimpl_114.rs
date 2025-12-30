// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl fmt :: Display for FromPathBufError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "PathBuf contains invalid UTF-8: {}" , self . path . display ()) } }
};
}
