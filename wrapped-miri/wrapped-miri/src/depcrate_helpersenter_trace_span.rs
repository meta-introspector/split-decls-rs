// Generated macro for enter_trace_span (macro)
macro_rules! Depcrate_helpersenter_trace_span {
() => {
// Module: crate::helpers
// Provides: {"enter_trace_span"}
// Dependencies: {}
# [doc = " Enters a [tracing::info_span] only if the \"tracing\" feature is enabled, otherwise does nothing."] # [doc = " This calls [rustc_const_eval::enter_trace_span] with [MiriMachine] as the first argument, which"] # [doc = " will in turn call [MiriMachine::enter_trace_span], which takes care of determining at compile"] # [doc = " time whether to trace or not (and supposedly the call is compiled out if tracing is disabled)."] # [doc = " Look at [rustc_const_eval::enter_trace_span] for complete documentation, examples and tips."] # [macro_export] macro_rules ! enter_trace_span { ($ ($ tt : tt) *) => { rustc_const_eval :: enter_trace_span ! ($ crate :: MiriMachine <'static >, $ ($ tt) *) } ; }
};
}
