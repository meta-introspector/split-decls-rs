// Generated macro for macro_8645 (macro)
macro_rules! Depcrate_partial_pub_fieldsmacro_8645 {
() => {
// Module: crate::partial_pub_fields
// Provides: {"macro_8645"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks whether some but not all fields of a `struct` are public."] # [doc = ""] # [doc = " Either make all fields of a type public, or make none of them public"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Most types should either be:"] # [doc = " * Abstract data types: complex objects with opaque implementation which guard"] # [doc = "   interior invariants and expose intentionally limited API to the outside world."] # [doc = " * Data:\u{2009}relatively simple objects which group a bunch of related attributes together,"] # [doc = "   but have no invariants."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " pub struct Color {"] # [doc = "     pub r: u8,"] # [doc = "     pub g: u8,"] # [doc = "     b: u8,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " pub struct Color {"] # [doc = "     pub r: u8,"] # [doc = "     pub g: u8,"] # [doc = "     pub b: u8,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub PARTIAL_PUB_FIELDS , restriction , "partial fields of a struct are public" }
};
}
