// Generated macro for __macro (module)
macro_rules! Depcrate__macro {
() => {
// Module: crate
// Provides: {"__macro"}
// Dependencies: {}
# [doc (hidden)] pub mod __macro { use gloo_utils :: format :: JsValueSerdeExt ; pub use js_sys :: Array ; pub use wasm_bindgen :: JsValue ; use wasm_bindgen :: UnwrapThrowExt ; pub fn table_with_data_and_columns < 'a > (data : impl serde :: Serialize , columns : impl IntoIterator < Item = & 'a str > ,) { let data = < JsValue as JsValueSerdeExt > :: from_serde (& data) . unwrap_throw () ; let columns = columns . into_iter () . map (JsValue :: from_str) . collect () ; crate :: externs :: table_with_data_and_columns (data , columns) ; } }
};
}
