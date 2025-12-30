// Generated macro for macro_1181 (macro)
macro_rules! Depcrate_cognitive_complexitymacro_1181 {
() => {
// Module: crate::cognitive_complexity
// Provides: {"macro_1181"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " We used to think it measured how hard a method is to understand."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Ideally, we would like to be able to measure how hard a function is"] # [doc = " to understand given its context (what we call its Cognitive Complexity)."] # [doc = " But that's not what this lint does. See \"Known problems\""] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The true Cognitive Complexity of a method is not something we can"] # [doc = " calculate using modern technology. This lint has been left in"] # [doc = " `restriction` so as to not mislead users into using this lint as a"] # [doc = " measurement tool."] # [doc = ""] # [doc = " For more detailed information, see [rust-clippy#3793](https://github.com/rust-lang/rust-clippy/issues/3793)"] # [doc = ""] # [doc = " ### Lints to consider instead of this"] # [doc = ""] # [doc = " * [`excessive_nesting`](https://rust-lang.github.io/rust-clippy/master/index.html#excessive_nesting)"] # [doc = " * [`too_many_lines`](https://rust-lang.github.io/rust-clippy/master/index.html#too_many_lines)"] # [clippy :: version = "1.35.0"] pub COGNITIVE_COMPLEXITY , restriction , "functions that should be split up into multiple functions" , @ eval_always = true }
};
}
