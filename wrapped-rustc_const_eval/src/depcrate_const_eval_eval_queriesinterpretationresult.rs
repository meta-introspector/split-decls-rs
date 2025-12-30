// Generated macro for InterpretationResult (trait)
macro_rules! Depcrate_const_eval_eval_queriesInterpretationResult {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"InterpretationResult"}
// Dependencies: {}
pub trait InterpretationResult < 'tcx > { # [doc = " This function takes the place where the result of the evaluation is stored"] # [doc = " and prepares it for returning it in the appropriate format needed by the specific"] # [doc = " evaluation query."] fn make_result (mplace : MPlaceTy < 'tcx > , ecx : & mut InterpCx < 'tcx , CompileTimeMachine < 'tcx > > ,) -> Self ; }
};
}
