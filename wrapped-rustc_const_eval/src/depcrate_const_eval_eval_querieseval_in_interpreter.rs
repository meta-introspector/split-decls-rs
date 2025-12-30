// Generated macro for eval_in_interpreter (function)
macro_rules! Depcrate_const_eval_eval_querieseval_in_interpreter {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"eval_in_interpreter"}
// Dependencies: {}
fn eval_in_interpreter < 'tcx , R : InterpretationResult < 'tcx > > (tcx : TyCtxt < 'tcx > , cid : GlobalId < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> Result < R , ErrorHandled > { let def = cid . instance . def . def_id () ; let is_static = tcx . is_static (def) ; let mut ecx = InterpCx :: new (tcx , tcx . def_span (def) , typing_env , CompileTimeMachine :: new (CanAccessMutGlobal :: from (is_static) , CheckAlignment :: Error) ,) ; let res = ecx . load_mir (cid . instance . def , cid . promoted) ; res . and_then (| body | eval_body_using_ecx (& mut ecx , cid , body)) . report_err () . map_err (| error | report_eval_error (& ecx , cid , error)) }
};
}
