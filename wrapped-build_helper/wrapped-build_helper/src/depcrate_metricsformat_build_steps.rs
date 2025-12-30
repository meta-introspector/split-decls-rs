// Generated macro for format_build_steps (function)
macro_rules! Depcrate_metricsformat_build_steps {
() => {
// Module: crate::metrics
// Provides: {"format_build_steps"}
// Dependencies: {}
# [doc = " Writes build steps into a nice indented table."] pub fn format_build_steps (root : & BuildStep) -> String { use std :: fmt :: Write ; let mut output = String :: new () ; for (level , step) in root . linearize_steps () { let label = format ! ("{}{}" , "." . repeat (level as usize) , escape_step_name (step)) ; writeln ! (output , "{label:.<65}{:>8.2}s" , step . duration . as_secs_f64 ()) . unwrap () ; } output }
};
}
