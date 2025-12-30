// Generated macro for impl_477 (impl)
macro_rules! Depcrateimpl_477 {
() => {
// Module: crate
// Provides: {"impl_477"}
// Dependencies: {}
impl From < Box < str > > for CompactString { # [inline] # [track_caller] fn from (b : Box < str >) -> Self { let s = b . into_string () ; let repr = Repr :: from_string (s , true) . unwrap_with_msg () ; CompactString (repr) } }
};
}
