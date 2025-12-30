// Generated macro for macro_3942 (macro)
macro_rules! Depcrate_loopsmacro_3942 {
() => {
// Module: crate::loops
// Provides: {"macro_3942"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for looping over the range of `0..len` of some"] # [doc = " collection just to get the values by index."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Just iterating the collection itself makes the intent"] # [doc = " more clear and is probably faster because it eliminates"] # [doc = " the bounds check that is done when indexing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " for i in 0..vec.len() {"] # [doc = "     println!(\"{}\", vec[i]);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let vec = vec!['a', 'b', 'c'];"] # [doc = " for i in vec {"] # [doc = "     println!(\"{}\", i);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_RANGE_LOOP , style , "for-looping over a range of indices where an iterator over items would do" }
};
}
