// Generated macro for impl_3074 (impl)
macro_rules! Depcrate_remove_constant_phisimpl_3074 {
() => {
// Module: crate::remove_constant_phis
// Provides: {"impl_3074"}
// Dependencies: {}
impl SolverState { fn new () -> Self { Self { absvals : FxHashMap :: default () , } } fn get (& self , actual : Value) -> AbstractValue { * self . absvals . get (& actual) . unwrap_or_else (| | panic ! ("SolverState::get: formal param {actual:?} is untracked?!")) } fn maybe_get (& self , actual : Value) -> Option < & AbstractValue > { self . absvals . get (& actual) } fn set (& mut self , actual : Value , lp : AbstractValue) { match self . absvals . insert (actual , lp) { Some (_old_lp) => { } None => panic ! ("SolverState::set: formal param {actual:?} is untracked?!") , } } }
};
}
