// Generated macro for macro_3333 (macro)
macro_rules! Depcrate_let_with_type_underscoremacro_3333 {
() => {
// Module: crate::let_with_type_underscore
// Provides: {"macro_3333"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects when a variable is declared with an explicit type of `_`."] # [doc = " ### Why is this bad?"] # [doc = " It adds noise, `: _` provides zero clarity or utility."] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let my_number: _ = 1;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let my_number = 1;"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub LET_WITH_TYPE_UNDERSCORE , complexity , "unneeded underscore type (`_`) in a variable declaration" }
};
}
