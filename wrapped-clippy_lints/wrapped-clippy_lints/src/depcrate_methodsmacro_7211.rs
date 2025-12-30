// Generated macro for macro_7211 (macro)
macro_rules! Depcrate_methodsmacro_7211 {
() => {
// Module: crate::methods
// Provides: {"macro_7211"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `.drain()` that clear the collection, immediately followed by a call to `.collect()`."] # [doc = ""] # [doc = " > \"Collection\" in this context refers to any type with a `drain` method:"] # [doc = " > `Vec`, `VecDeque`, `BinaryHeap`, `HashSet`,`HashMap`, `String`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `mem::take` is faster as it avoids the allocation."] # [doc = " When using `mem::take`, the old collection is replaced with an empty one and ownership of"] # [doc = " the old collection is returned."] # [doc = ""] # [doc = " ### Known issues"] # [doc = " `mem::take(&mut vec)` is almost equivalent to `vec.drain(..).collect()`, except that"] # [doc = " it also moves the **capacity**. The user might have explicitly written it this way"] # [doc = " to keep the capacity on the original `Vec`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn remove_all(v: &mut Vec<i32>) -> Vec<i32> {"] # [doc = "     v.drain(..).collect()"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::mem;"] # [doc = " fn remove_all(v: &mut Vec<i32>) -> Vec<i32> {"] # [doc = "     mem::take(v)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub DRAIN_COLLECT , perf , "calling `.drain(..).collect()` to move all elements into a new collection" }
};
}
