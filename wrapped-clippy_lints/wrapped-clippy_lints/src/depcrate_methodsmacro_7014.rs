// Generated macro for macro_7014 (macro)
macro_rules! Depcrate_methodsmacro_7014 {
() => {
// Module: crate::methods
// Provides: {"macro_7014"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " As the counterpart to `or_fun_call`, this lint looks for unnecessary"] # [doc = " lazily evaluated closures on `Option` and `Result`."] # [doc = ""] # [doc = " This lint suggests changing the following functions, when eager evaluation results in"] # [doc = " simpler code:"] # [doc = "  - `unwrap_or_else` to `unwrap_or`"] # [doc = "  - `and_then` to `and`"] # [doc = "  - `or_else` to `or`"] # [doc = "  - `get_or_insert_with` to `get_or_insert`"] # [doc = "  - `ok_or_else` to `ok_or`"] # [doc = "  - `then` to `then_some` (for msrv >= 1.62.0)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using eager evaluation is shorter and simpler in some cases."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " It is possible, but not recommended for `Deref` and `Index` to have"] # [doc = " side effects. Eagerly evaluating them can change the semantics of the program."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let opt: Option<u32> = None;"] # [doc = ""] # [doc = " opt.unwrap_or_else(|| 42);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let opt: Option<u32> = None;"] # [doc = ""] # [doc = " opt.unwrap_or(42);"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub UNNECESSARY_LAZY_EVALUATIONS , style , "using unnecessary lazy evaluation, which can be replaced with simpler eager evaluation" }
};
}
