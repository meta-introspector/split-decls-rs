// Generated macro for impl_31 (impl)
macro_rules! Depcrate_oomimpl_31 {
() => {
// Module: crate::oom
// Provides: {"impl_31"}
// Dependencies: {}
impl Drop for R { fn drop (& mut self) { self . 0 . 0 . fetch_sub (1 , AcqRel) ; panic_if (| | ! self . 0 . 1 && rand :: random :: < u8 > () . is_multiple_of (11)) ; } }
};
}
