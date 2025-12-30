// Generated macro for SolverState (struct)
macro_rules! Depcrate_remove_constant_phisSolverState {
() => {
// Module: crate::remove_constant_phis
// Provides: {"SolverState"}
// Dependencies: {}
# [doc = " Solver state.  This holds a AbstractValue for each formal parameter, except"] # [doc = " for those from the entry block."] struct SolverState { absvals : FxHashMap < Value , AbstractValue > , }
};
}
