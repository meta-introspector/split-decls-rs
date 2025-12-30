// Generated macro for prob (function)
macro_rules! Depcrate_optionprob {
() => {
// Module: crate::option
// Provides: {"prob"}
// Dependencies: {}
# [doc = " Creates a `Probability` from some value that is convertible into it."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the converted to probability would lie"] # [doc = " outside interval `[0.0, 1.0]`. Consult the `Into` (or `From`)"] # [doc = " implementations for more details."] pub fn prob (from : impl Into < Probability >) -> Probability { from . into () }
};
}
