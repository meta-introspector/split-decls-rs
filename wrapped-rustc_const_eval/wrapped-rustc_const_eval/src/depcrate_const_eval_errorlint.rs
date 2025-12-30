// Generated macro for lint (function)
macro_rules! Depcrate_const_eval_errorlint {
() => {
// Module: crate::const_eval::error
// Provides: {"lint"}
// Dependencies: {}
# [doc = " Emit a lint from a const-eval situation, with a backtrace."] # [allow (unused)] pub (super) fn lint < 'tcx , L > (tcx : TyCtxtAt < 'tcx > , machine : & CompileTimeMachine < 'tcx > , lint : & 'static rustc_session :: lint :: Lint , decorator : impl FnOnce (Vec < errors :: FrameNote >) -> L ,) where L : for < 'a > rustc_errors :: LintDiagnostic < 'a , () > , { let (span , frames) = get_span_and_frames (tcx , & machine . stack) ; tcx . emit_node_span_lint (lint , machine . best_lint_scope (* tcx) , span , decorator (frames)) ; }
};
}
