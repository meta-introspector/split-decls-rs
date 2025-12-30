// Generated macro for impl_131 (impl)
macro_rules! Depcrate_standardimpl_131 {
() => {
// Module: crate::standard
// Provides: {"impl_131"}
// Dependencies: {}
impl < W : io :: Write > Standard < NoColor < W > > { # [doc = " Return a standard printer with a default configuration that writes"] # [doc = " matches to the given writer."] # [doc = ""] # [doc = " The writer can be any implementation of `io::Write`. With this"] # [doc = " constructor, the printer will never emit colors."] pub fn new_no_color (wtr : W) -> Standard < NoColor < W > > { StandardBuilder :: new () . build_no_color (wtr) } }
};
}
