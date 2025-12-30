// Generated macro for macro_1289 (macro)
macro_rules! Depcrate_copy_iteratormacro_1289 {
() => {
// Module: crate::copy_iterator
// Provides: {"macro_1289"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for types that implement `Copy` as well as"] # [doc = " `Iterator`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Implicit copies can be confusing when working with"] # [doc = " iterator combinators."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Copy, Clone)]"] # [doc = " struct Countdown(u8);"] # [doc = ""] # [doc = " impl Iterator for Countdown {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " let a: Vec<_> = my_iterator.take(1).collect();"] # [doc = " let b: Vec<_> = my_iterator.collect();"] # [doc = " ```"] # [clippy :: version = "1.30.0"] pub COPY_ITERATOR , pedantic , "implementing `Iterator` on a `Copy` type" }
};
}
