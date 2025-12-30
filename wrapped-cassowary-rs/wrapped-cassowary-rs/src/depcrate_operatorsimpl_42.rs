// Generated macro for impl_42 (impl)
macro_rules! Depcrate_operatorsimpl_42 {
() => {
// Module: crate::operators
// Provides: {"impl_42"}
// Dependencies: {}
impl ops :: Sub < Variable > for Variable { type Output = Expression ; fn sub (self , v : Variable) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0) , Term :: new (v , - 1.0)] , 0.0) } }
};
}
