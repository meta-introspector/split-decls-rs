// Generated macro for ImportString (struct)
macro_rules! Depcrate_astImportString {
() => {
// Module: crate::ast
// Provides: {"ImportString"}
// Dependencies: {}
# [doc = " The type of a static string being imported"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub struct ImportString { # [doc = " The visibility of this static string in Rust"] pub vis : syn :: Visibility , # [doc = " The type specified by the user, which we only use to show an error if the wrong type is used."] pub ty : syn :: Type , # [doc = " The name of the shim function used to access this static"] pub shim : Ident , # [doc = " The name of this static on the Rust side"] pub rust_name : Ident , # [doc = " Path to wasm_bindgen"] pub wasm_bindgen : Path , # [doc = " Path to js_sys"] pub js_sys : Path , # [doc = " The string to export."] pub string : String , # [doc = " Version of `thread_local`."] pub thread_local : ThreadLocal , }
};
}
