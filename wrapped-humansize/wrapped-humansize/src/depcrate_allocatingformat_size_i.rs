// Generated macro for format_size_i (function)
macro_rules! Depcrate_allocatingformat_size_i {
() => {
// Module: crate::allocating
// Provides: {"format_size_i"}
// Dependencies: {}
pub fn format_size_i (input : impl ToF64 , options : impl AsRef < FormatSizeOptions >) -> String { format ! ("{}" , ISizeFormatter :: new (input , options)) }
};
}
