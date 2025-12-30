// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
impl From < Utf8PathBuf > for Arc < Utf8Path > { fn from (path : Utf8PathBuf) -> Arc < Utf8Path > { let arc : Arc < Path > = Arc :: from (path . 0) ; let ptr = Arc :: into_raw (arc) as * const Utf8Path ; unsafe { Arc :: from_raw (ptr) } } }
};
}
