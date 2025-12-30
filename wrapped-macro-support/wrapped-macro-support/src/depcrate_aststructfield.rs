// Generated macro for StructField (struct)
macro_rules! Depcrate_astStructField {
() => {
// Module: crate::ast
// Provides: {"StructField"}
// Dependencies: {}
# [doc = " The field of a struct"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct StructField { # [doc = " The name of the field in Rust code"] pub rust_name : syn :: Member , # [doc = " The name of the field in JS code"] pub js_name : String , # [doc = " The name of the struct this field is part of"] pub struct_name : Ident , # [doc = " Whether this value is read-only to JS"] pub readonly : bool , # [doc = " The type of this field"] pub ty : syn :: Type , # [doc = " The name of the getter shim for this field"] pub getter : Ident , # [doc = " The name of the setter shim for this field"] pub setter : Ident , # [doc = " The doc comments on this field, if any"] pub comments : Vec < String > , # [doc = " Whether to generate a typescript definition for this field"] pub generate_typescript : bool , # [doc = " Whether to generate jsdoc documentation for this field"] pub generate_jsdoc : bool , # [doc = " The span of the `#[wasm_bindgen(getter_with_clone)]` attribute applied"] # [doc = " to this field, if any."] # [doc = ""] # [doc = " If this is `Some`, the auto-generated getter for this field must clone"] # [doc = " the field instead of copying it."] pub getter_with_clone : Option < Span > , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , }
};
}
