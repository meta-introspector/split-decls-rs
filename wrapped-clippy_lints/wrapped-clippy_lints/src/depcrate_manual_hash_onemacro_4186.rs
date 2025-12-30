// Generated macro for macro_4186 (macro)
macro_rules! Depcrate_manual_hash_onemacro_4186 {
() => {
// Module: crate::manual_hash_one
// Provides: {"macro_4186"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for cases where [`BuildHasher::hash_one`] can be used."] # [doc = ""] # [doc = " [`BuildHasher::hash_one`]: https://doc.rust-lang.org/std/hash/trait.BuildHasher.html#method.hash_one"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is more concise to use the `hash_one` method."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::hash::{BuildHasher, Hash, Hasher};"] # [doc = " use std::collections::hash_map::RandomState;"] # [doc = ""] # [doc = " let s = RandomState::new();"] # [doc = " let value = vec![1, 2, 3];"] # [doc = ""] # [doc = " let mut hasher = s.build_hasher();"] # [doc = " value.hash(&mut hasher);"] # [doc = " let hash = hasher.finish();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::hash::BuildHasher;"] # [doc = " use std::collections::hash_map::RandomState;"] # [doc = ""] # [doc = " let s = RandomState::new();"] # [doc = " let value = vec![1, 2, 3];"] # [doc = ""] # [doc = " let hash = s.hash_one(&value);"] # [doc = " ```"] # [clippy :: version = "1.75.0"] pub MANUAL_HASH_ONE , complexity , "manual implementations of `BuildHasher::hash_one`" }
};
}
