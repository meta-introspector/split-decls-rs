// Generated macro for impl_245 (impl)
macro_rules! Depcrate_next_solver_genericsimpl_245 {
() => {
// Module: crate::next_solver::generics
// Provides: {"impl_245"}
// Dependencies: {}
impl < T > ops :: Index < T > for Generics where GenericParams : ops :: Index < T > , { type Output = < GenericParams as ops :: Index < T > > :: Output ; fn index (& self , index : T) -> & Self :: Output { & self . params [index] } }
};
}
