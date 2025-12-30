// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_util_rangeintimpl_1246 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1246"}
// Dependencies: {}
impl Composite < i8 > { pub (crate) const fn to_rint < const MIN : i128 , const MAX : i128 > (self ,) -> ri8 < MIN , MAX > { # [cfg (not (debug_assertions))] { ri8 { val : self . val } } # [cfg (debug_assertions)] { ri8 { val : self . val , min : self . min , max : self . max } } } }
};
}
