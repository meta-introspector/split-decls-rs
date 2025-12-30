// Generated macro for Color (struct)
macro_rules! DepcrateColor {
() => {
// Module: crate
// Provides: {"Color"}
// Dependencies: {}
# [doc = " Mixin a clap argument for colored output selection"] # [allow (clippy :: exhaustive_structs)] # [derive (Copy , Clone , Default , Debug , PartialEq , Eq , clap :: Args)] # [command (about = None , long_about = None)] pub struct Color { # [doc = " Controls when to use color."] # [arg (long , default_value_t = ColorChoice :: Auto , value_name = "WHEN" , value_enum , global = true)] pub color : ColorChoice , }
};
}
