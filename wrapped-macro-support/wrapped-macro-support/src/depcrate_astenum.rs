// Generated macro for Enum (struct)
macro_rules! Depcrate_astEnum {
() => {
// Module: crate::ast
// Provides: {"Enum"}
// Dependencies: {}
# [doc = " The metadata for an Enum"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct Enum { # [doc = " The name of this enum in Rust code"] pub rust_name : Ident , # [doc = " The name of this enum in JS code"] pub js_name : String , # [doc = " Whether the variant values and hole are signed, meaning that they"] # [doc = " represent the bits of a `i32` value."] pub signed : bool , # [doc = " The variants provided by this enum"] pub variants : Vec < Variant > , # [doc = " The doc comments on this enum, if any"] pub comments : Vec < String > , # [doc = " The value to use for a `none` variant of the enum"] pub hole : u32 , # [doc = " Whether to generate a typescript definition for this enum"] pub generate_typescript : bool , # [doc = " The namespace to export the enum through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , }
};
}
