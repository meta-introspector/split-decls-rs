// Generated macro for impl_29 (impl)
macro_rules! Depcrate_operatorsimpl_29 {
() => {
// Module: crate::operators
// Provides: {"impl_29"}
// Dependencies: {}
impl ops :: Add < Variable > for f64 { type Output = Expression ; fn add (self , v : Variable) -> Expression { Expression :: new (vec ! [Term :: new (v , 1.0)] , self) } }
};
}
