// Generated macro for impl_1229 (impl)
macro_rules! Depcrate_util_rangeintimpl_1229 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1229"}
// Dependencies: {}
impl < const MIN : i128 , const MAX : i128 > ri32 < MIN , MAX > { # [inline] pub (crate) fn without_bounds (self ,) -> ri64 < { i64 :: MIN as i128 } , { i64 :: MAX as i128 } > { ri64 :: rfrom (self) } }
};
}
