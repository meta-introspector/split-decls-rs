// Generated macro for make_format (function)
macro_rules! Depcrate_allocatingmake_format {
() => {
// Module: crate::allocating
// Provides: {"make_format"}
// Dependencies: {}
pub fn make_format < T : ToF64 + Unsigned > (options : impl AsRef < FormatSizeOptions > ,) -> impl Fn (T) -> String { make_format_i (options) }
};
}
