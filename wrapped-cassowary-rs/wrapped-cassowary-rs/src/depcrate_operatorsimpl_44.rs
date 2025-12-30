// Generated macro for impl_44 (impl)
macro_rules! Depcrate_operatorsimpl_44 {
() => {
// Module: crate::operators
// Provides: {"impl_44"}
// Dependencies: {}
impl ops :: Sub < Variable > for Term { type Output = Expression ; fn sub (self , v : Variable) -> Expression { Expression :: new (vec ! [self , Term :: new (v , - 1.0)] , 0.0) } }
};
}
