// Generated macro for FromSolverError (trait)
macro_rules! Depcrate_traits_engineFromSolverError {
() => {
// Module: crate::traits::engine
// Provides: {"FromSolverError"}
// Dependencies: {}
pub trait FromSolverError < 'tcx , E > : Debug + 'tcx { fn from_solver_error (infcx : & InferCtxt < 'tcx > , error : E) -> Self ; }
};
}
