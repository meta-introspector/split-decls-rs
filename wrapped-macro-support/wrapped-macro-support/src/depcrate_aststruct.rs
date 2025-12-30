// Generated macro for Struct (struct)
macro_rules! Depcrate_astStruct {
() => {
// Module: crate::ast
// Provides: {"Struct"}
// Dependencies: {}
# [doc = " Information about a Struct being exported"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct Struct { # [doc = " The name of the struct in Rust code"] pub rust_name : Ident , # [doc = " The name of the struct in JS code"] pub js_name : String , # [doc = " All the fields of this struct to export"] pub fields : Vec < StructField > , # [doc = " The doc comments on this struct, if provided"] pub comments : Vec < String > , # [doc = " Whether this struct is inspectable (provides toJSON/toString properties to JS)"] pub is_inspectable : bool , # [doc = " Whether to generate a typescript definition for this struct"] pub generate_typescript : bool , # [doc = " The namespace to export the struct through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , }
};
}
