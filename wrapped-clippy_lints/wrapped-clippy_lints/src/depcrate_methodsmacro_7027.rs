// Generated macro for macro_7027 (macro)
macro_rules! Depcrate_methodsmacro_7027 {
() => {
// Module: crate::methods
// Provides: {"macro_7027"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `str::splitn` (or `str::rsplitn`) where using `str::split` would be the same."] # [doc = " ### Why is this bad?"] # [doc = " The function `split` is simpler and there is no performance difference in these cases, considering"] # [doc = " that both functions return a lazy iterator."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let str = \"key=value=add\";"] # [doc = " let _ = str.splitn(3, '=').next().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let str = \"key=value=add\";"] # [doc = " let _ = str.split('=').next().unwrap();"] # [doc = " ```"] # [clippy :: version = "1.59.0"] pub NEEDLESS_SPLITN , complexity , "usages of `str::splitn` that can be replaced with `str::split`" }
};
}
