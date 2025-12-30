// Generated macro for ImportFunction (struct)
macro_rules! Depcrate_astImportFunction {
() => {
// Module: crate::ast
// Provides: {"ImportFunction"}
// Dependencies: {}
# [doc = " A function being imported from JS"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct ImportFunction { # [doc = " The full signature of the function"] pub function : Function , # [doc = " The name rust code will use"] pub rust_name : Ident , # [doc = " The type being returned"] pub js_ret : Option < syn :: Type > , # [doc = " Whether to catch JS exceptions"] pub catch : bool , # [doc = " Whether the function is variadic on the JS side"] pub variadic : bool , # [doc = " Whether the function should use structural type checking"] pub structural : bool , # [doc = " Causes the Builder (See cli-support::js::binding::Builder) to error out if"] # [doc = " it finds itself generating code for a function with this signature"] pub assert_no_shim : bool , # [doc = " The kind of function being imported"] pub kind : ImportFunctionKind , # [doc = " The shim name to use in the generated code. The 'shim' is a function that appears in"] # [doc = " the generated JS as a wrapper around the actual function to import, performing any"] # [doc = " necessary conversions (EG adding a try/catch to change a thrown error into a Result)"] pub shim : Ident , # [doc = " The doc comment on this import, if one is provided"] pub doc_comment : String , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , # [doc = " Path to wasm_bindgen_futures"] pub wasm_bindgen_futures : Path , }
};
}
