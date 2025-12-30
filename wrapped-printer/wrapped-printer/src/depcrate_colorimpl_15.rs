// Generated macro for impl_15 (impl)
macro_rules! Depcrate_colorimpl_15 {
() => {
// Module: crate::color
// Provides: {"impl_15"}
// Dependencies: {}
impl UserColorSpec { # [doc = " Convert this user provided color specification to a specification that"] # [doc = " can be used with `termcolor`. This drops the type of this specification"] # [doc = " (where the type indicates where the color is applied in the standard"] # [doc = " printer, e.g., to the file path or the line numbers, etc.)."] pub fn to_color_spec (& self) -> ColorSpec { let mut spec = ColorSpec :: default () ; self . value . merge_into (& mut spec) ; spec } }
};
}
