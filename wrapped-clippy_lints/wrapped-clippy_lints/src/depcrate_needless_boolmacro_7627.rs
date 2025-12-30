// Generated macro for macro_7627 (macro)
macro_rules! Depcrate_needless_boolmacro_7627 {
() => {
// Module: crate::needless_bool
// Provides: {"macro_7627"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions of the form `if c { x = true } else { x = false }`"] # [doc = " (or vice versa) and suggest assigning the variable directly from the"] # [doc = " condition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " # fn must_keep(x: i32, y: i32) -> bool { x == y }"] # [doc = " # let x = 32; let y = 10;"] # [doc = " # let mut skip: bool;"] # [doc = " if must_keep(x, y) {"] # [doc = "     skip = false;"] # [doc = " } else {"] # [doc = "     skip = true;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " # fn must_keep(x: i32, y: i32) -> bool { x == y }"] # [doc = " # let x = 32; let y = 10;"] # [doc = " # let mut skip: bool;"] # [doc = " skip = !must_keep(x, y);"] # [doc = " ```"] # [clippy :: version = "1.71.0"] pub NEEDLESS_BOOL_ASSIGN , complexity , "setting the same boolean variable in both branches of an if-statement" }
};
}
