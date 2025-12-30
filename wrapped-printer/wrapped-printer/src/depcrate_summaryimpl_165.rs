// Generated macro for impl_165 (impl)
macro_rules! Depcrate_summaryimpl_165 {
() => {
// Module: crate::summary
// Provides: {"impl_165"}
// Dependencies: {}
impl < W : io :: Write > Summary < NoColor < W > > { # [doc = " Return a summary printer with a default configuration that writes"] # [doc = " matches to the given writer."] # [doc = ""] # [doc = " The writer can be any implementation of `io::Write`. With this"] # [doc = " constructor, the printer will never emit colors."] # [doc = ""] # [doc = " The default configuration uses the `Count` summary mode."] pub fn new_no_color (wtr : W) -> Summary < NoColor < W > > { SummaryBuilder :: new () . build_no_color (wtr) } }
};
}
