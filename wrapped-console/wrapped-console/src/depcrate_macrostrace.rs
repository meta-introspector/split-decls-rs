// Generated macro for trace (macro)
macro_rules! Depcrate_macrostrace {
() => {
// Module: crate::macros
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Calls `console.trace()`"] # [macro_export] macro_rules ! trace { ($ ($ arg : expr) ,+) => { $ crate :: externs :: trace (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
