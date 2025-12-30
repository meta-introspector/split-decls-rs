// Generated macro for make_format_i (function)
macro_rules! Depcrate_allocatingmake_format_i {
() => {
// Module: crate::allocating
// Provides: {"make_format_i"}
// Dependencies: {}
pub fn make_format_i < T : ToF64 > (options : impl AsRef < FormatSizeOptions >) -> impl Fn (T) -> String { move | val | -> String { format_size_i (val , & options) } }
};
}
