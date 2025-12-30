// Generated macro for console (macro)
macro_rules! Depcrate_console_dbgconsole {
() => {
// Module: crate::console_dbg
// Provides: {"console"}
// Dependencies: {}
# [doc = " A macro similar to [`dbg!`] that logs [`JsValue`][wasm_bindgen::JsValue]s to console."] # [doc = ""] # [doc = " See the [stdlib documentation][std::dbg] to learn more. This macro calls `console.log`"] # [doc = " instead of `eprintln!` for `JsValue`s. The formatting is done by the browser. If you want"] # [doc = " [`Debug`][std::fmt::Debug] implementation to be used instead, consider using [`console_dbg`]"] # [macro_export] macro_rules ! console { () => { $ crate :: log ! (:: std :: format ! ("%c[{}:{}] " , :: std :: file ! () , :: std :: line ! ()) , "font-weight: bold") ; } ; ($ val : expr $ (,) ?) => { { let v = $ val ; $ crate :: __console_inner ! (v $ val) } } ; ($ ($ val : expr) ,+ $ (,) ?) => { ($ ($ crate :: console ! ($ val)) ,+,) } ; }
};
}
