// Generated macro for impl_501 (impl)
macro_rules! Depcrateimpl_501 {
() => {
// Module: crate
// Provides: {"impl_501"}
// Dependencies: {}
impl FromIterator < CompactString > for Cow < '_ , str > { fn from_iter < T : IntoIterator < Item = CompactString > > (iter : T) -> Self { String :: from_iter (iter) . into () } }
};
}
