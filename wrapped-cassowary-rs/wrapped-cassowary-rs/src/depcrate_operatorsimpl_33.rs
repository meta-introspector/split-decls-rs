// Generated macro for impl_33 (impl)
macro_rules! Depcrate_operatorsimpl_33 {
() => {
// Module: crate::operators
// Provides: {"impl_33"}
// Dependencies: {}
impl ops :: Add < Variable > for Term { type Output = Expression ; fn add (self , v : Variable) -> Expression { Expression :: new (vec ! [self , Term :: new (v , 1.0)] , 0.0) } }
};
}
