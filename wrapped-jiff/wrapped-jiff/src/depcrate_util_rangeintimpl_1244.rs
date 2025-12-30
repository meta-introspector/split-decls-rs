// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_util_rangeintimpl_1244 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1244"}
// Dependencies: {}
impl < T > Composite < T > { # [inline] pub (crate) fn map < U > (self , map : impl Fn (T) -> U) -> Composite < U > { # [cfg (not (debug_assertions))] { Composite { val : map (self . val) } } # [cfg (debug_assertions)] { Composite { val : map (self . val) , min : map (self . min) , max : map (self . max) , } } } # [inline] pub (crate) fn zip2 < U > (self , other : Composite < U >) -> Composite < (T , U) > { # [cfg (not (debug_assertions))] { Composite { val : (self . val , other . val) } } # [cfg (debug_assertions)] { Composite { val : (self . val , other . val) , min : (self . min , other . min) , max : (self . max , other . max) , } } } }
};
}
