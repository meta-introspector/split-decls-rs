// Generated macro for debug (macro)
macro_rules! Depcrate_macrosdebug {
() => {
// Module: crate::macros
// Provides: {"debug"}
// Dependencies: {}
# [doc = " Calls `console.debug()`"] # [macro_export] macro_rules ! debug { ($ ($ arg : expr) ,+) => { $ crate :: externs :: debug (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
