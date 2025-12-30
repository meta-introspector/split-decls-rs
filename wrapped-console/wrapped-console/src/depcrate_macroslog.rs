// Generated macro for log (macro)
macro_rules! Depcrate_macroslog {
() => {
// Module: crate::macros
// Provides: {"log"}
// Dependencies: {}
# [doc = " Calls `console.log()`"] # [macro_export] macro_rules ! log { ($ ($ arg : expr) ,+) => { $ crate :: externs :: log (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
