// Generated macro for report_eval_error (function)
macro_rules! Depcrate_const_eval_eval_queriesreport_eval_error {
() => {
// Module: crate::const_eval::eval_queries
// Provides: {"report_eval_error"}
// Dependencies: {}
# [inline (never)] fn report_eval_error < 'tcx > (ecx : & InterpCx < 'tcx , CompileTimeMachine < 'tcx > > , cid : GlobalId < 'tcx > , error : InterpErrorInfo < 'tcx > ,) -> ErrorHandled { let (error , backtrace) = error . into_parts () ; backtrace . print_backtrace () ; let instance = with_no_trimmed_paths ! (cid . instance . to_string ()) ; super :: report (ecx , error , DUMMY_SP , | | super :: get_span_and_frames (ecx . tcx , ecx . stack ()) , | diag , span , frames | { let num_frames = frames . len () ; diag . code (E0080) ; diag . span_label (span , crate :: fluent_generated :: const_eval_error) ; for frame in frames { diag . subdiagnostic (frame) ; } diag . arg ("instance" , instance) ; diag . arg ("num_frames" , num_frames) ; } ,) }
};
}
