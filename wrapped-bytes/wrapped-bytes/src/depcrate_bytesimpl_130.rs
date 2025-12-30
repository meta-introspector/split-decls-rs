// Generated macro for impl_130 (impl)
macro_rules! Depcrate_bytesimpl_130 {
() => {
// Module: crate::bytes
// Provides: {"impl_130"}
// Dependencies: {}
impl FromIterator < u8 > for Bytes { fn from_iter < T : IntoIterator < Item = u8 > > (into_iter : T) -> Self { Vec :: from_iter (into_iter) . into () } }
};
}
