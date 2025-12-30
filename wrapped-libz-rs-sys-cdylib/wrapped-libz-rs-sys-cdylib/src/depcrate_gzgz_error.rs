// Generated macro for gz_error (function)
macro_rules! Depcrate_gzgz_error {
() => {
// Module: crate::gz
// Provides: {"gz_error"}
// Dependencies: {}
unsafe fn gz_error (state : & mut GzState , err_msg : Option < (c_int , & str) >) { if ! state . msg . is_null () { unsafe { deallocate_cstr (state . msg . cast_mut ()) } ; state . msg = ptr :: null_mut () ; } match err_msg { None => { state . err = Z_OK ; } Some ((err , msg)) => { if err != Z_OK && err != Z_BUF_ERROR { state . have = 0 ; } state . err = err ; if err == Z_MEM_ERROR { return ; } let sep = ": " ; let buf = & mut [0u8 ; 27] ; state . msg = match state . source { Source :: Path (path) => unsafe { gz_strcat (& [CStr :: from_ptr (path) . to_str () . unwrap () , sep , msg]) } , Source :: Fd (fd) => unsafe { gz_strcat (& [fd_path (buf , fd) . to_str () . unwrap () , sep , msg]) } , } ; if state . msg . is_null () { state . err = Z_MEM_ERROR ; } } } }
};
}
