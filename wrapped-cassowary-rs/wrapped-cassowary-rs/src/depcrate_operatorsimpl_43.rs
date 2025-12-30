// Generated macro for impl_43 (impl)
macro_rules! Depcrate_operatorsimpl_43 {
() => {
// Module: crate::operators
// Provides: {"impl_43"}
// Dependencies: {}
impl ops :: Sub < Term > for Variable { type Output = Expression ; fn sub (self , t : Term) -> Expression { Expression :: new (vec ! [Term :: new (self , 1.0) , - t] , 0.0) } }
};
}
