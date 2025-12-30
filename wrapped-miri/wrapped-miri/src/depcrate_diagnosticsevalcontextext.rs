// Generated macro for EvalContextExt (trait)
macro_rules! Depcrate_diagnosticsEvalContextExt {
() => {
// Module: crate::diagnostics
// Provides: {"EvalContextExt"}
// Dependencies: {}
pub trait EvalContextExt < 'tcx > : crate :: MiriInterpCxExt < 'tcx > { fn emit_diagnostic (& self , e : NonHaltingDiagnostic) { let this = self . eval_context_ref () ; this . machine . emit_diagnostic (e) ; } # [doc = " We had a panic in Miri itself, try to print something useful."] fn handle_ice (& self) { eprintln ! () ; eprintln ! ("Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:") ; let this = self . eval_context_ref () ; let stacktrace = this . generate_stacktrace () ; report_msg (DiagLevel :: Note , "the place in the program where the ICE was triggered" . to_string () , vec ! [] , vec ! [] , vec ! [] , & stacktrace , Some (this . active_thread ()) , & this . machine ,) ; } }
};
}
