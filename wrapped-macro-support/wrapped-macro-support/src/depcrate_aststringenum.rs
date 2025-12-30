// Generated macro for StringEnum (struct)
macro_rules! Depcrate_astStringEnum {
() => {
// Module: crate::ast
// Provides: {"StringEnum"}
// Dependencies: {}
# [doc = " The metadata for a String Enum"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct StringEnum { # [doc = " The Rust enum's visibility"] pub vis : syn :: Visibility , # [doc = " The Rust enum's identifiers"] pub name : Ident , # [doc = " The name of this string enum in JS/TS code"] pub js_name : String , # [doc = " The Rust identifiers for the variants"] pub variants : Vec < Ident > , # [doc = " The JS string values of the variants"] pub variant_values : Vec < String > , # [doc = " The doc comments on this enum, if any"] pub comments : Vec < String > , # [doc = " Attributes to apply to the Rust enum"] pub rust_attrs : Vec < syn :: Attribute > , # [doc = " Whether to generate a typescript definition for this enum"] pub generate_typescript : bool , # [doc = " The namespace to export the enum through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , }
};
}
