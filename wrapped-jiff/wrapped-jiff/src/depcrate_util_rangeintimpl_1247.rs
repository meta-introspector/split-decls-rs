// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_util_rangeintimpl_1247 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1247"}
// Dependencies: {}
impl Composite < i16 > { pub (crate) const fn to_rint < const MIN : i128 , const MAX : i128 > (self ,) -> ri16 < MIN , MAX > { # [cfg (not (debug_assertions))] { ri16 { val : self . val } } # [cfg (debug_assertions)] { ri16 { val : self . val , min : self . min , max : self . max } } } }
};
}
