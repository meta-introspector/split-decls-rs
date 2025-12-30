// Generated macro for ConditionVisitor (struct)
macro_rules! Depcrate_diagnostics_conflict_errorsConditionVisitor {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"ConditionVisitor"}
// Dependencies: {}
# [doc = " Given a set of spans representing statements initializing the relevant binding, visit all the"] # [doc = " function expressions looking for branching code paths that *do not* initialize the binding."] struct ConditionVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , spans : Vec < Span > , name : String , errors : Vec < (Span , String) > , }
};
}
