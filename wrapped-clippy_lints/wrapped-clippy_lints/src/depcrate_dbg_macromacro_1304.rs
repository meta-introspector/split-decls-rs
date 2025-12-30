// Generated macro for macro_1304 (macro)
macro_rules! Depcrate_dbg_macromacro_1304 {
() => {
// Module: crate::dbg_macro
// Provides: {"macro_1304"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of the [`dbg!`](https://doc.rust-lang.org/std/macro.dbg.html) macro."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The `dbg!` macro is intended as a debugging tool. It should not be present in released"] # [doc = " software or committed to a version control system."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " dbg!(true)"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " true"] # [doc = " ```"] # [clippy :: version = "1.34.0"] pub DBG_MACRO , restriction , "`dbg!` macro is intended as a debugging tool" }
};
}
