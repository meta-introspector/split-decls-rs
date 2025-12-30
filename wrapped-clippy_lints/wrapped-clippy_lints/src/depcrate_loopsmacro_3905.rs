// Generated macro for macro_3905 (macro)
macro_rules! Depcrate_loopsmacro_3905 {
() => {
// Module: crate::loops
// Provides: {"macro_3905"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for uses of the `enumerate` method where the index is unused (`_`)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The index from `.enumerate()` is immediately dropped."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " let v = vec![1, 2, 3, 4];"] # [doc = " for (_, x) in v.iter().enumerate() {"] # [doc = "     println!(\"{x}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " let v = vec![1, 2, 3, 4];"] # [doc = " for x in v.iter() {"] # [doc = "     println!(\"{x}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.75.0"] pub UNUSED_ENUMERATE_INDEX , style , "using `.enumerate()` and immediately dropping the index" }
};
}
