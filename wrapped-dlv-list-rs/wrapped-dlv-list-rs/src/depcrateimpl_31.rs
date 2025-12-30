// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < T > ops :: Index < Index < T > > for VecList < T > { type Output = T ; fn index (& self , index : Index < T >) -> & Self :: Output { self . get (index) . expect ("expected entry at index") } }
};
}
