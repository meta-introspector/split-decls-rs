// Generated macro for print_version (function)
macro_rules! Depcrateprint_version {
() => {
// Module: crate
// Provides: {"print_version"}
// Dependencies: {}
pub fn print_version () { unsafe { println ! ("HarfBuzz version {:?}" , str :: from_utf8 (CStr :: from_ptr (hb_version_string ()) . to_bytes ())) ; } }
};
}
