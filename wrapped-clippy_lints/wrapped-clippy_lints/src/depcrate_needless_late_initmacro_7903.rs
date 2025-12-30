// Generated macro for macro_7903 (macro)
macro_rules! Depcrate_needless_late_initmacro_7903 {
() => {
// Module: crate::needless_late_init
// Provides: {"macro_7903"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for late initializations that can be replaced by a `let` statement"] # [doc = " with an initializer."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Assigning in the `let` statement is less repetitive."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a;"] # [doc = " a = 1;"] # [doc = ""] # [doc = " let b;"] # [doc = " match 3 {"] # [doc = "     0 => b = \"zero\","] # [doc = "     1 => b = \"one\","] # [doc = "     _ => b = \"many\","] # [doc = " }"] # [doc = ""] # [doc = " let c;"] # [doc = " if true {"] # [doc = "     c = 1;"] # [doc = " } else {"] # [doc = "     c = -1;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = 1;"] # [doc = ""] # [doc = " let b = match 3 {"] # [doc = "     0 => \"zero\","] # [doc = "     1 => \"one\","] # [doc = "     _ => \"many\","] # [doc = " };"] # [doc = ""] # [doc = " let c = if true {"] # [doc = "     1"] # [doc = " } else {"] # [doc = "     -1"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.59.0"] pub NEEDLESS_LATE_INIT , style , "late initializations that can be replaced by a `let` statement with an initializer" }
};
}
