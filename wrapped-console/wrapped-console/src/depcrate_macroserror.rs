// Generated macro for error (macro)
macro_rules! Depcrate_macroserror {
() => {
// Module: crate::macros
// Provides: {"error"}
// Dependencies: {}
# [doc = " Calls `console.error()`"] # [macro_export] macro_rules ! error { ($ ($ arg : expr) ,+) => { $ crate :: externs :: error (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
