// Generated macro for impl_678 (impl)
macro_rules! Depcrate_genericsimpl_678 {
() => {
// Module: crate::generics
// Provides: {"impl_678"}
// Dependencies: {}
impl < T > ops :: Index < T > for Generics where GenericParams : ops :: Index < T > , { type Output = < GenericParams as ops :: Index < T > > :: Output ; fn index (& self , index : T) -> & Self :: Output { & self . params [index] } }
};
}
