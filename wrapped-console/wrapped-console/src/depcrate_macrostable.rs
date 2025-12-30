// Generated macro for table (macro)
macro_rules! Depcrate_macrostable {
() => {
// Module: crate::macros
// Provides: {"table"}
// Dependencies: {}
# [doc = " Calls `console.table()`"] # [doc = ""] # [doc = " Since in most cases, this takes in an object, instead of, say a string literal/variable,"] # [doc = " we use [`serde`](https://serde.rs) to serialize the passed data object into"] # [doc = " [`JsValue`][wasm_bindgen::JsValue]."] # [doc = ""] # [doc = " An `IntoIterator<Item = &str>` can be passed to specify the columns."] # [macro_export] macro_rules ! table { ($ data : expr) => { $ crate :: externs :: table_with_data ($ crate :: __macro :: JsValue :: from ($ data)) ; } ; ($ data : expr , $ columns : expr) => { $ crate :: __macro :: table_with_data_and_columns ($ data , $ columns) ; } ; }
};
}
