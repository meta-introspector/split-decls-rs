// Generated macro for group (macro)
macro_rules! Depcrate_macrosgroup {
() => {
// Module: crate::macros
// Provides: {"group"}
// Dependencies: {}
# [doc = " Calls `console.group()`"] # [doc = ""] # [doc = " In order to call `console.groupCollapsed`, prefix the arguments with `collapsed`."] # [macro_export] macro_rules ! group { ($ ($ arg : expr) ,+) => { $ crate :: externs :: group (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } ; (collapsed $ ($ arg : expr) ,+) => { $ crate :: externs :: group_collapsed (:: std :: boxed :: Box :: from ([$ ($ crate :: __macro :: JsValue :: from ($ arg) ,) +])) ; } ; }
};
}
