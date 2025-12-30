// Generated macro for impl_479 (impl)
macro_rules! Depcrateimpl_479 {
() => {
// Module: crate
// Provides: {"impl_479"}
// Dependencies: {}
impl From < CompactString > for Cow < '_ , str > { # [inline] fn from (s : CompactString) -> Self { if let Some (s) = s . as_static_str () { Self :: Borrowed (s) } else { Self :: Owned (s . into_string ()) } } }
};
}
