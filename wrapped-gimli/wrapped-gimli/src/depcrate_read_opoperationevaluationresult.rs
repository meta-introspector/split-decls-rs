// Generated macro for OperationEvaluationResult (enum)
macro_rules! Depcrate_read_opOperationEvaluationResult {
() => {
// Module: crate::read::op
// Provides: {"OperationEvaluationResult"}
// Dependencies: {}
# [derive (Debug)] enum OperationEvaluationResult < R : Reader > { Piece , Incomplete , Complete { location : Location < R > } , Waiting (EvaluationWaiting < R > , EvaluationResult < R >) , }
};
}
