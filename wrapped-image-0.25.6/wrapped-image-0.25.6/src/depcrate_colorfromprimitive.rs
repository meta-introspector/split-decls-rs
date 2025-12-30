// Generated macro for FromPrimitive (trait)
macro_rules! Depcrate_colorFromPrimitive {
() => {
// Module: crate::color
// Provides: {"FromPrimitive"}
// Dependencies: {}
# [doc = " Convert from one pixel component type to another. For example, convert from `u8` to `f32` pixel values."] pub trait FromPrimitive < Component > { # [doc = " Converts from any pixel component type to this type."] fn from_primitive (component : Component) -> Self ; }
};
}
