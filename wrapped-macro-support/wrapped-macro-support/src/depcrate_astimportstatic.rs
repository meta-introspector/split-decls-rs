// Generated macro for ImportStatic (struct)
macro_rules! Depcrate_astImportStatic {
() => {
// Module: crate::ast
// Provides: {"ImportStatic"}
// Dependencies: {}
# [doc = " The type of a static being imported"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct ImportStatic { # [doc = " The visibility of this static in Rust"] pub vis : syn :: Visibility , # [doc = " The type of static being imported"] pub ty : syn :: Type , # [doc = " The name of the shim function used to access this static"] pub shim : Ident , # [doc = " The name of this static on the Rust side"] pub rust_name : Ident , # [doc = " The name of this static on the JS side"] pub js_name : String , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , # [doc = " Version of `thread_local`, if any."] pub thread_local : Option < ThreadLocal > , }
};
}
