// Generated macro for impl_32 (impl)
macro_rules! Depcrate_operatorsimpl_32 {
() => {
// Module: crate::operators
// Provides: {"impl_32"}
// Dependencies: {}
impl ops :: Add < Term > for Variable { type Output = Expression ; fn add (self , t : Term) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0) , t] , 0.0) } }
};
}
