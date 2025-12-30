// Generated macro for macro_11178 (macro)
macro_rules! Depcrate_unused_unitmacro_11178 {
() => {
// Module: crate::unused_unit
// Provides: {"macro_11178"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unit (`()`) expressions that can be removed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Such expressions add no value, but can make the code"] # [doc = " less readable. Depending on formatting they can make a `break` or `return`"] # [doc = " statement look like a function call."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn return_unit() -> () {"] # [doc = "     ()"] # [doc = " }"] # [doc = " ```"] # [doc = " is equivalent to"] # [doc = " ```no_run"] # [doc = " fn return_unit() {}"] # [doc = " ```"] # [clippy :: version = "1.31.0"] pub UNUSED_UNIT , style , "needless unit expression" }
};
}
