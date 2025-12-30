// Generated macro for impl_40 (impl)
macro_rules! Depcrate_operatorsimpl_40 {
() => {
// Module: crate::operators
// Provides: {"impl_40"}
// Dependencies: {}
impl ops :: Sub < Variable > for f64 { type Output = Expression ; fn sub (self , v : Variable) -> Expression { Expression :: new (vec ! [Term :: new (v , - 1.0)] , self) } }
};
}
