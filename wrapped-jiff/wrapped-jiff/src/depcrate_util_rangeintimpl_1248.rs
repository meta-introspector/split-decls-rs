// Generated macro for impl_1248 (impl)
macro_rules! Depcrate_util_rangeintimpl_1248 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1248"}
// Dependencies: {}
impl Composite < i32 > { pub (crate) const fn to_rint < const MIN : i128 , const MAX : i128 > (self ,) -> ri32 < MIN , MAX > { # [cfg (not (debug_assertions))] { ri32 { val : self . val } } # [cfg (debug_assertions)] { ri32 { val : self . val , min : self . min , max : self . max } } } }
};
}
