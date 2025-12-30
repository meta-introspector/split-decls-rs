// Generated macro for dump_error (function)
macro_rules! Depcratedump_error {
() => {
// Module: crate
// Provides: {"dump_error"}
// Dependencies: {}
# [allow (dead_code)] unsafe fn dump_error () { let err = ERR_get_error () ; let lib = ERR_GET_LIB (err) ; let reason = ERR_GET_REASON (err) ; let func = ERR_GET_FUNC (err) ; let mut buffer = [0u8 ; 256] ; ERR_error_string (err , buffer . as_mut_ptr () . cast ()) ; let error_msg = CStr :: from_bytes_with_nul_unchecked (& buffer) ; eprintln ! ("Raw Error -- {error_msg:?}\nErr: {err}, Lib: {lib}, Reason: {reason}, Func: {func}") ; }
};
}
