// Generated macro for impl_88 (impl)
macro_rules! Depcrateimpl_88 {
() => {
// Module: crate
// Provides: {"impl_88"}
// Dependencies: {}
impl From < & '_ Utf8Path > for Arc < Utf8Path > { fn from (path : & Utf8Path) -> Arc < Utf8Path > { let arc : Arc < Path > = Arc :: from (AsRef :: < Path > :: as_ref (path)) ; let ptr = Arc :: into_raw (arc) as * const Utf8Path ; unsafe { Arc :: from_raw (ptr) } } }
};
}
