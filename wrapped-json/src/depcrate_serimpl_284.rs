// Generated macro for impl_284 (impl)
macro_rules! Depcrate_serimpl_284 {
() => {
// Module: crate::ser
// Provides: {"impl_284"}
// Dependencies: {}
impl < 'a > PrettyFormatter < 'a > { # [doc = " Construct a pretty printer formatter that defaults to using two spaces for indentation."] pub fn new () -> Self { PrettyFormatter :: with_indent (b"  ") } # [doc = " Construct a pretty printer formatter that uses the `indent` string for indentation."] pub fn with_indent (indent : & 'a [u8]) -> Self { PrettyFormatter { current_indent : 0 , has_value : false , indent , } } }
};
}
