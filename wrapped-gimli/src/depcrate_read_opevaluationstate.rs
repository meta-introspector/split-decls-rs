// Generated macro for EvaluationState (enum)
macro_rules! Depcrate_read_opEvaluationState {
() => {
// Module: crate::read::op
// Provides: {"EvaluationState"}
// Dependencies: {}
# [derive (Debug)] enum EvaluationState < R : Reader > { Start (Option < u64 >) , Ready , Error (Error) , Complete , Waiting (EvaluationWaiting < R >) , }
};
}
