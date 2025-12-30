// Generated macro for impl_11331 (impl)
macro_rules! Depcrate_useless_conversionimpl_11331 {
() => {
// Module: crate::useless_conversion
// Provides: {"impl_11331"}
// Dependencies: {}
impl MethodOrFunction { # [doc = " Maps the argument position in `pos` to the parameter position."] # [doc = " For methods, `self` is skipped."] fn param_pos (self , pos : usize) -> usize { match self { MethodOrFunction :: Method => pos + 1 , MethodOrFunction :: Function => pos , } } }
};
}
