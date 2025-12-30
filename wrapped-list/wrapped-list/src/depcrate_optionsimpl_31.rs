// Generated macro for impl_31 (impl)
macro_rules! Depcrate_optionsimpl_31 {
() => {
// Module: crate::options
// Provides: {"impl_31"}
// Dependencies: {}
impl ListFormatterOptions { # [doc = " Constructs a new [`ListFormatterOptions`] struct."] pub const fn default () -> Self { Self { length : None } } # [doc = " Auguments the struct with the set [`ListLength`]."] pub const fn with_length (mut self , length : ListLength) -> Self { self . length = Some (length) ; self } }
};
}
