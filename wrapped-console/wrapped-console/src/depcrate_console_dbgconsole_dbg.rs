// Generated macro for console_dbg (macro)
macro_rules! Depcrate_console_dbgconsole_dbg {
() => {
// Module: crate::console_dbg
// Provides: {"console_dbg"}
// Dependencies: {}
# [doc = " A macro similar to [`dbg!`] to log to browser console."] # [doc = ""] # [doc = " See the [stdlib documentation][std::dbg] to learn more. This macro calls `console.log`"] # [doc = " instead of `eprintln!`. This macro passing the values to [`console`] after formatting them using"] # [doc = " the [`Debug`][std::fmt::Debug] implementation."] # [macro_export] macro_rules ! console_dbg { () => { $ crate :: console ! () } ; ($ val : expr $ (,) ?) => { { let v : $ crate :: __macro :: JsValue = :: std :: format ! ("{:?}" , $ val) . into () ; $ crate :: __console_inner ! (v $ val) } } ; ($ ($ val : expr) ,+ $ (,) ?) => { ($ ($ crate :: console_dbg ! ($ val)) ,+,) } ; }
};
}
