// Generated macro for new_signal (function)
macro_rules! Depcrate_leavesnew_signal {
() => {
// Module: crate::leaves
// Provides: {"new_signal"}
// Dependencies: {}
pub fn new_signal < D : DataType > (n : Member < 'static > , data : D :: Signal) -> Signal < D > { Signal { name : n , arguments : vec ! () , anns : Annotations :: new () , data : data } }
};
}
