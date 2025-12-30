// Generated macro for macro_7207 (macro)
macro_rules! Depcrate_methodsmacro_7207 {
() => {
// Module: crate::methods
// Provides: {"macro_7207"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions collecting an iterator when collect"] # [doc = " is not needed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `collect` causes the allocation of a new data structure,"] # [doc = " when this allocation may not be needed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let iterator = vec![1].into_iter();"] # [doc = " let len = iterator.collect::<Vec<_>>().len();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let iterator = vec![1].into_iter();"] # [doc = " let len = iterator.count();"] # [doc = " ```"] # [clippy :: version = "1.30.0"] pub NEEDLESS_COLLECT , nursery , "collecting an iterator when collect is not needed" }
};
}
