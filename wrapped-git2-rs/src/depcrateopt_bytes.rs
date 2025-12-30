// Generated macro for opt_bytes (function)
macro_rules! Depcrateopt_bytes {
() => {
// Module: crate
// Provides: {"opt_bytes"}
// Dependencies: {}
unsafe fn opt_bytes < 'a , T > (_anchor : & 'a T , c : * const libc :: c_char) -> Option < & 'a [u8] > { if c . is_null () { None } else { Some (CStr :: from_ptr (c) . to_bytes ()) } }
};
}
