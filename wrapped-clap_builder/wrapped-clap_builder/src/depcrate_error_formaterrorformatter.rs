// Generated macro for ErrorFormatter (trait)
macro_rules! Depcrate_error_formatErrorFormatter {
() => {
// Module: crate::error::format
// Provides: {"ErrorFormatter"}
// Dependencies: {}
# [doc = " Defines how to format an error for displaying to the user"] pub trait ErrorFormatter : Sized { # [doc = " Stylize the error for the terminal"] fn format_error (error : & crate :: error :: Error < Self >) -> StyledStr ; }
};
}
