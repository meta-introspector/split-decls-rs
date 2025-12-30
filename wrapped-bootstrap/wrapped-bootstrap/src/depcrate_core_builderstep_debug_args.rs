// Generated macro for step_debug_args (function)
macro_rules! Depcrate_core_builderstep_debug_args {
() => {
// Module: crate::core::builder
// Provides: {"step_debug_args"}
// Dependencies: {}
# [doc = " Renders `step` using its `Debug` implementation and extract the field arguments out of it."] fn step_debug_args < S : Step > (step : & S) -> String { let step_dbg_repr = format ! ("{step:?}") ; match (step_dbg_repr . find ('{') , step_dbg_repr . rfind ('}')) { (Some (brace_start) , Some (brace_end)) => { step_dbg_repr [brace_start + 1 .. brace_end - 1] . trim () . to_string () } _ => String :: new () , } }
};
}
