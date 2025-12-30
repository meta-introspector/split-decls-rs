// Generated macro for pretty_print_step (function)
macro_rules! Depcrate_core_builderpretty_print_step {
() => {
// Module: crate::core::builder
// Provides: {"pretty_print_step"}
// Dependencies: {}
fn pretty_print_step < S : Step > (step : & S) -> String { format ! ("{} {{ {} }}" , pretty_step_name ::< S > () , step_debug_args (step)) }
};
}
