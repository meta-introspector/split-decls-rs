// Generated macro for impl_164 (impl)
macro_rules! Depcrate_summaryimpl_164 {
() => {
// Module: crate::summary
// Provides: {"impl_164"}
// Dependencies: {}
impl < W : WriteColor > Summary < W > { # [doc = " Return a summary printer with a default configuration that writes"] # [doc = " matches to the given writer."] # [doc = ""] # [doc = " The writer should be an implementation of `termcolor::WriteColor`"] # [doc = " and not just a bare implementation of `io::Write`. To use a normal"] # [doc = " `io::Write` implementation (simultaneously sacrificing colors), use"] # [doc = " the `new_no_color` constructor."] # [doc = ""] # [doc = " The default configuration uses the `Count` summary mode."] pub fn new (wtr : W) -> Summary < W > { SummaryBuilder :: new () . build (wtr) } }
};
}
