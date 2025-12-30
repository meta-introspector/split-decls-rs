// Generated macro for dirxml (macro)
macro_rules! Depcrate_macrosdirxml {
() => {
// Module: crate::macros
// Provides: {"dirxml"}
// Dependencies: {}
# [doc = " Calls `console.dirxml()`"] # [doc = " ## Example"] # [doc = " ```no_run"] # [doc = " # use gloo_console::dirxml;"] # [doc = " # use js_sys::Array;"] # [doc = " dirxml!(Array::of2(&1.into(), &2.into()));"] # [doc = " ```"] # [macro_export] macro_rules ! dirxml { ($ arg : expr) => { $ crate :: externs :: dirxml (&$ crate :: __macro :: JsValue :: from ($ arg)) ; } ; }
};
}
