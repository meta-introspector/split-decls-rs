// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a > From < & 'a [u8] > for TextRef < 'a > { fn from (d : & 'a [u8]) -> Self { let d = if d [d . len () - 1] == b'\n' { & d [.. d . len () - 1] } else { d } ; TextRef (d) } }
};
}
