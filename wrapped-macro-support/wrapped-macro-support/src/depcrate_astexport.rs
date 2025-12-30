// Generated macro for Export (struct)
macro_rules! Depcrate_astExport {
() => {
// Module: crate::ast
// Provides: {"Export"}
// Dependencies: {}
# [doc = " A rust to js interface. Allows interaction with rust objects/functions"] # [doc = " from javascript."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct Export { # [doc = " Comments extracted from the rust source."] pub comments : Vec < String > , # [doc = " The rust function"] pub function : Function , # [doc = " The class name in JS this is attached to"] pub js_class : Option < String > , # [doc = " The namespace to export the item through, if any"] pub js_namespace : Option < Vec < String > > , # [doc = " The kind (static, named, regular)"] pub method_kind : MethodKind , # [doc = " The type of `self` (either `self`, `&self`, or `&mut self`)"] pub method_self : Option < MethodSelf > , # [doc = " The struct name, in Rust, this is attached to"] pub rust_class : Option < Ident > , # [doc = " The name of the rust function/method on the rust side."] pub rust_name : Ident , # [doc = " Whether or not this function should be flagged as the Wasm start"] # [doc = " function."] pub start : bool , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , # [doc = " Path to wasm_bindgen_futures"] pub wasm_bindgen_futures : Path , }
};
}
