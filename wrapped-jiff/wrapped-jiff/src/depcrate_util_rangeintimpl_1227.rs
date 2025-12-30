// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_util_rangeintimpl_1227 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1227"}
// Dependencies: {}
impl < const MIN : i128 , const MAX : i128 > ri8 < MIN , MAX > { # [inline] pub (crate) fn without_bounds (self ,) -> ri64 < { i64 :: MIN as i128 } , { i64 :: MAX as i128 } > { ri64 :: rfrom (self) } }
};
}
