// Generated macro for Program (struct)
macro_rules! Depcrate_astProgram {
() => {
// Module: crate::ast
// Provides: {"Program"}
// Dependencies: {}
# [doc = " An abstract syntax tree representing a rust program. Contains"] # [doc = " extra information for joining up this rust code with javascript."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct Program { # [doc = " rust -> js interfaces"] pub exports : Vec < Export > , # [doc = " js -> rust interfaces"] pub imports : Vec < Import > , # [doc = " linked-to modules"] pub linked_modules : Vec < ImportModule > , # [doc = " rust enums"] pub enums : Vec < Enum > , # [doc = " rust structs"] pub structs : Vec < Struct > , # [doc = " custom typescript sections to be included in the definition file"] pub typescript_custom_sections : Vec < LitOrExpr > , # [doc = " Inline JS snippets"] pub inline_js : Vec < String > , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , # [doc = " Path to js_sys"] pub js_sys : Path , # [doc = " Path to wasm_bindgen_futures"] pub wasm_bindgen_futures : Path , }
};
}
