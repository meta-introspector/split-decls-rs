// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_util_rangeintimpl_1231 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1231"}
// Dependencies: {}
impl < const MIN : i128 , const MAX : i128 > ri128 < MIN , MAX > { # [inline] pub (crate) fn without_bounds (self) -> ri128 < { i128 :: MIN } , { i128 :: MAX } > { ri128 :: rfrom (self) } }
};
}
