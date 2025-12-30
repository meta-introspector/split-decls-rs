// Generated macro for acquire_header_and_release (function)
macro_rules! Depcrate_exportacquire_header_and_release {
() => {
// Module: crate::export
// Provides: {"acquire_header_and_release"}
// Dependencies: {}
# [inline (never)] pub fn acquire_header_and_release (s : & Str) { unsafe { acquire () } ; istr (s) ; timestamp (make_formatter ()) ; unsafe { release () } ; }
};
}
