// Generated macro for dir (macro)
macro_rules! Depcrate_macrosdir {
() => {
// Module: crate::macros
// Provides: {"dir"}
// Dependencies: {}
# [doc = " Calls `console.dir()`"] # [doc = " ## Example"] # [doc = " ```no_run"] # [doc = " # use gloo_console::dir;"] # [doc = " # use js_sys::Array;"] # [doc = " dir!(Array::of2(&1.into(), &2.into()));"] # [doc = " ```"] # [macro_export] macro_rules ! dir { ($ arg : expr) => { $ crate :: externs :: dir (&$ crate :: __macro :: JsValue :: from ($ arg)) ; } ; }
};
}
