// Generated macro for description (function)
macro_rules! Depcrate_errordescription {
() => {
// Module: crate::error
// Provides: {"description"}
// Dependencies: {}
fn description (code : c_int) -> Option < & 'static str > { match code { libc :: EINVAL => Some ("`newp` is not `NULL`, and `newlen` is too large or too \
             small. Alternatively, `*oldlenp` is too large or too \
             small; in this case as much data as possible are read \
             despite the error." ,) , libc :: ENOENT => { Some ("`name` or `mib` specifies an unknown/invalid value.") } libc :: EPERM => Some ("Attempt to read or write `void` value, or attempt to \
             write read-only value." ,) , libc :: EAGAIN => Some ("A memory allocation failure occurred.") , libc :: EFAULT => Some ("An interface with side effects failed in some way not \
             directly related to `mallctl*()` read/write processing." ,) , _ => None , } }
};
}
