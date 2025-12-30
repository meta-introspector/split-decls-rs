// Generated macro for impl_1230 (impl)
macro_rules! Depcrate_util_rangeintimpl_1230 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1230"}
// Dependencies: {}
impl < const MIN : i128 , const MAX : i128 > ri64 < MIN , MAX > { # [inline] pub (crate) fn without_bounds (self ,) -> ri64 < { i64 :: MIN as i128 } , { i64 :: MAX as i128 } > { ri64 :: rfrom (self) } }
};
}
