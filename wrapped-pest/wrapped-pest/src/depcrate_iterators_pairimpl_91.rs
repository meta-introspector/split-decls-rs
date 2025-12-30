// Generated macro for impl_91 (impl)
macro_rules! Depcrate_iterators_pairimpl_91 {
() => {
// Module: crate::iterators::pair
// Provides: {"impl_91"}
// Dependencies: {}
impl < 'i , R : RuleType > Pairs < 'i , R > { # [doc = " Create a new `Pairs` iterator containing just the single `Pair`."] pub fn single (pair : Pair < 'i , R >) -> Self { let end = pair . pair () ; pairs :: new (pair . queue , pair . input , Some (pair . line_index) , pair . start , end ,) } }
};
}
