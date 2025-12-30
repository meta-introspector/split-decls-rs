// Generated macro for impl_31 (impl)
macro_rules! Depcrate_operatorsimpl_31 {
() => {
// Module: crate::operators
// Provides: {"impl_31"}
// Dependencies: {}
impl ops :: Add < Variable > for Variable { type Output = Expression ; fn add (self , v : Variable) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0) , Term :: new (v , 1.0)] , 0.0) } }
};
}
