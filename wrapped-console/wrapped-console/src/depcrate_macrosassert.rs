// Generated macro for assert (macro)
macro_rules! Depcrate_macrosassert {
() => {
// Module: crate::macros
// Provides: {"assert"}
// Dependencies: {}
# [doc = " Calls `console.assert()`"] # [macro_export] macro_rules ! assert { ($ assertion : expr , $ ($ arg : expr) ,+) => { $ crate :: externs :: assert ($ assertion , :: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
