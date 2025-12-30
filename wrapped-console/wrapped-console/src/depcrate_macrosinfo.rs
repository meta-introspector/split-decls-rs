// Generated macro for info (macro)
macro_rules! Depcrate_macrosinfo {
() => {
// Module: crate::macros
// Provides: {"info"}
// Dependencies: {}
# [doc = " Calls `console.info()`"] # [macro_export] macro_rules ! info { ($ ($ arg : expr) ,+) => { $ crate :: externs :: info (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
