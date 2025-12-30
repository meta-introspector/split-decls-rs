// Generated macro for impl_1249 (impl)
macro_rules! Depcrate_util_rangeintimpl_1249 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1249"}
// Dependencies: {}
impl Composite < i64 > { pub (crate) const fn to_rint < const MIN : i128 , const MAX : i128 > (self ,) -> ri64 < MIN , MAX > { # [cfg (not (debug_assertions))] { ri64 { val : self . val } } # [cfg (debug_assertions)] { ri64 { val : self . val , min : self . min , max : self . max } } } pub (crate) fn try_to_rint < const MIN : i128 , const MAX : i128 > (self , what : & 'static str ,) -> Result < ri64 < MIN , MAX > , Error > { # [cfg (not (debug_assertions))] { if ! ri64 :: < MIN , MAX > :: contains (self . val) { return Err (ri64 :: < MIN , MAX > :: error (what , self . val)) ; } Ok (ri64 { val : self . val }) } # [cfg (debug_assertions)] { if ! ri64 :: < MIN , MAX > :: contains (self . val) { return Err (ri64 :: < MIN , MAX > :: error (what , self . val)) ; } Ok (ri64 { val : self . val , min : self . min . clamp (MIN as i64 , MAX as i64) , max : self . max . clamp (MIN as i64 , MAX as i64) , }) } } }
};
}
