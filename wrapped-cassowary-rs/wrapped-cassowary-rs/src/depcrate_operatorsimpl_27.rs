// Generated macro for impl_27 (impl)
macro_rules! Depcrate_operatorsimpl_27 {
() => {
// Module: crate::operators
// Provides: {"impl_27"}
// Dependencies: {}
impl ops :: Add < f64 > for Variable { type Output = Expression ; fn add (self , v : f64) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0)] , v) } }
};
}
