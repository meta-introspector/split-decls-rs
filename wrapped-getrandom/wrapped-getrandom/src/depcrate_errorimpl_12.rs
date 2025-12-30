// Generated macro for impl_12 (impl)
macro_rules! Depcrate_errorimpl_12 {
() => {
// Module: crate::error
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dbg = f . debug_struct ("Error") ; if let Some (errno) = self . raw_os_error () { dbg . field ("os_error" , & errno) ; # [cfg (feature = "std")] dbg . field ("description" , & std :: io :: Error :: from_raw_os_error (errno)) ; } else if let Some (desc) = self . internal_desc () { dbg . field ("internal_code" , & self . 0 . get ()) ; dbg . field ("description" , & desc) ; } else { dbg . field ("unknown_code" , & self . 0 . get ()) ; } dbg . finish () } }
};
}
