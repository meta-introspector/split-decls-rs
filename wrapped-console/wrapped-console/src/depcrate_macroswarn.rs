// Generated macro for warn (macro)
macro_rules! Depcrate_macroswarn {
() => {
// Module: crate::macros
// Provides: {"warn"}
// Dependencies: {}
# [doc = " Calls `console.warn()`"] # [macro_export] macro_rules ! warn { ($ ($ arg : expr) ,+) => { $ crate :: externs :: warn (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } }
};
}
