// Generated macro for impl_474 (impl)
macro_rules! Depcrateimpl_474 {
() => {
// Module: crate
// Provides: {"impl_474"}
// Dependencies: {}
impl From < String > for CompactString { # [inline] # [track_caller] fn from (s : String) -> Self { let repr = Repr :: from_string (s , true) . unwrap_with_msg () ; CompactString (repr) } }
};
}
