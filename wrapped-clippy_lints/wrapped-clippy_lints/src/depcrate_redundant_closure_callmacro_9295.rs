// Generated macro for macro_9295 (macro)
macro_rules! Depcrate_redundant_closure_callmacro_9295 {
() => {
// Module: crate::redundant_closure_call
// Provides: {"macro_9295"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects closures called in the same expression where they"] # [doc = " are defined."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is unnecessarily adding to the expression's"] # [doc = " complexity."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = (|| 42)();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = 42;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub REDUNDANT_CLOSURE_CALL , complexity , "throwaway closures called in the expression they are defined" }
};
}
