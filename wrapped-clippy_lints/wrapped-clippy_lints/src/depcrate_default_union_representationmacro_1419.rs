// Generated macro for macro_1419 (macro)
macro_rules! Depcrate_default_union_representationmacro_1419 {
() => {
// Module: crate::default_union_representation
// Provides: {"macro_1419"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Displays a warning when a union is declared with the default representation (without a `#[repr(C)]` attribute)."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Unions in Rust have unspecified layout by default, despite many people thinking that they"] # [doc = " lay out each field at the start of the union (like C does). That is, there are no guarantees"] # [doc = " about the offset of the fields for unions with multiple non-ZST fields without an explicitly"] # [doc = " specified layout. These cases may lead to undefined behavior in unsafe blocks."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " union Foo {"] # [doc = "     a: i32,"] # [doc = "     b: u32,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _x: u32 = unsafe {"] # [doc = "         Foo { a: 0_i32 }.b // Undefined behavior: `b` is allowed to be padding"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[repr(C)]"] # [doc = " union Foo {"] # [doc = "     a: i32,"] # [doc = "     b: u32,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let _x: u32 = unsafe {"] # [doc = "         Foo { a: 0_i32 }.b // Now defined behavior, this is just an i32 -> u32 transmute"] # [doc = "     };"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub DEFAULT_UNION_REPRESENTATION , restriction , "unions without a `#[repr(C)]` attribute" }
};
}
