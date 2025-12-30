// Generated macro for gethostname (function)
macro_rules! Depcrate_hostnamegethostname {
() => {
// Module: crate::hostname
// Provides: {"gethostname"}
// Dependencies: {}
# [cfg (unix)] fn gethostname () -> io :: Result < OsString > { use std :: os :: unix :: ffi :: OsStringExt ; let limit = unsafe { libc :: sysconf (libc :: _SC_HOST_NAME_MAX) } ; if limit == - 1 { return Err (io :: Error :: last_os_error ()) ; } let Ok (maxlen) = usize :: try_from (limit) else { let msg = format ! ("host name max limit ({}) overflowed usize" , limit) ; return Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) ; } ; let mut buf = vec ! [0 ; maxlen] ; let rc = unsafe { libc :: gethostname (buf . as_mut_ptr () . cast :: < libc :: c_char > () , maxlen) } ; if rc == - 1 { return Err (io :: Error :: last_os_error ()) ; } let Some (zeropos) = buf . iter () . position (| & b | b == 0) else { let msg = "could not find NUL terminator in hostname" ; return Err (io :: Error :: new (io :: ErrorKind :: Other , msg)) ; } ; buf . truncate (zeropos) ; buf . shrink_to_fit () ; Ok (OsString :: from_vec (buf)) }
};
}
