// Generated macro for BackendStyle (trait)
macro_rules! Depcrate_styleBackendStyle {
() => {
// Module: crate::style
// Provides: {"BackendStyle"}
// Dependencies: {}
# [doc = " The style data for the backend drawing API"] pub trait BackendStyle { # [doc = " Get the color of current style"] fn color (& self) -> BackendColor ; # [doc = " Get the stroke width of current style"] fn stroke_width (& self) -> u32 { 1 } }
};
}
