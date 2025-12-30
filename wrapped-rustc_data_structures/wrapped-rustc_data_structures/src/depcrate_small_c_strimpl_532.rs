// Generated macro for impl_532 (impl)
macro_rules! Depcrate_small_c_strimpl_532 {
() => {
// Module: crate::small_c_str
// Provides: {"impl_532"}
// Dependencies: {}
impl < 'a > FromIterator < & 'a str > for SmallCStr { fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { let mut data = iter . into_iter () . flat_map (| s | s . as_bytes ()) . copied () . collect :: < SmallVec < _ > > () ; data . push (0) ; if let Err (e) = ffi :: CStr :: from_bytes_with_nul (& data) { panic ! ("The iterator {data:?} cannot be converted into a CStr: {e}") ; } Self { data } } }
};
}
