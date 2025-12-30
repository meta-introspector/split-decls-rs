// Generated macro for macro_3945 (macro)
macro_rules! Depcrate_loopsmacro_3945 {
() => {
// Module: crate::loops
// Provides: {"macro_3945"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for loops on `x.next()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `next()` returns either `Some(value)` if there was a"] # [doc = " value, or `None` otherwise. The insidious thing is that `Option<_>`"] # [doc = " implements `IntoIterator`, so that possibly one value will be iterated,"] # [doc = " leading to some hard to find bugs. No one will want to write such code"] # [doc = " [except to win an Underhanded Rust"] # [doc = " Contest](https://www.reddit.com/r/rust/comments/3hb0wm/underhanded_rust_contest/cu5yuhr)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " for x in y.next() {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITER_NEXT_LOOP , correctness , "for-looping over `_.next()` which is probably not intended" }
};
}
