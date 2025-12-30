// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: Error :: * ; match self { Io (err) => err . fmt (f) , Utf8 (name) => write ! (f , "unsupported non-utf8 file name: {}" , name . to_string_lossy () ,) , Empty => f . write_str ("no source files found") , } } }
};
}
