// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (errno) = self . raw_os_error () { cfg_if ! { if # [cfg (feature = "std")] { std :: io :: Error :: from_raw_os_error (errno) . fmt (f) } else { write ! (f , "OS Error: {errno}") } } } else if let Some (desc) = self . internal_desc () { f . write_str (desc) } else { write ! (f , "Unknown Error: {}" , self . 0 . get ()) } } }
};
}
