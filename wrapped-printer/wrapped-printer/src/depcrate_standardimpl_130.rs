// Generated macro for impl_130 (impl)
macro_rules! Depcrate_standardimpl_130 {
() => {
// Module: crate::standard
// Provides: {"impl_130"}
// Dependencies: {}
impl < W : WriteColor > Standard < W > { # [doc = " Return a standard printer with a default configuration that writes"] # [doc = " matches to the given writer."] # [doc = ""] # [doc = " The writer should be an implementation of `termcolor::WriteColor`"] # [doc = " and not just a bare implementation of `io::Write`. To use a normal"] # [doc = " `io::Write` implementation (simultaneously sacrificing colors), use"] # [doc = " the `new_no_color` constructor."] pub fn new (wtr : W) -> Standard < W > { StandardBuilder :: new () . build (wtr) } }
};
}
