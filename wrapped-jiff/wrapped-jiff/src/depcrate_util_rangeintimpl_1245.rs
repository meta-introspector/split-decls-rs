// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_util_rangeintimpl_1245 {
() => {
// Module: crate::util::rangeint
// Provides: {"impl_1245"}
// Dependencies: {}
impl < T , U > Composite < (T , U) > { # [inline] pub (crate) fn unzip2 (self) -> (Composite < T > , Composite < U >) { # [cfg (not (debug_assertions))] { (Composite { val : self . val . 0 } , Composite { val : self . val . 1 }) } # [cfg (debug_assertions)] { (Composite { val : self . val . 0 , min : self . min . 0 , max : self . max . 0 , } , Composite { val : self . val . 1 , min : self . min . 1 , max : self . max . 1 , } ,) } } }
};
}
