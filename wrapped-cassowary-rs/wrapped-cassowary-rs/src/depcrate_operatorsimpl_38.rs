// Generated macro for impl_38 (impl)
macro_rules! Depcrate_operatorsimpl_38 {
() => {
// Module: crate::operators
// Provides: {"impl_38"}
// Dependencies: {}
impl ops :: Sub < f64 > for Variable { type Output = Expression ; fn sub (self , v : f64) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0)] , - v) } }
};
}
