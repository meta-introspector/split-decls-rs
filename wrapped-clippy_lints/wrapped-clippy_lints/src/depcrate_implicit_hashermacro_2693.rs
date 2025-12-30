// Generated macro for macro_2693 (macro)
macro_rules! Depcrate_implicit_hashermacro_2693 {
() => {
// Module: crate::implicit_hasher
// Provides: {"macro_2693"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for public `impl` or `fn` missing generalization"] # [doc = " over different hashers and implicitly defaulting to the default hashing"] # [doc = " algorithm (`SipHash`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `HashMap` or `HashSet` with custom hashers cannot be"] # [doc = " used with them."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Suggestions for replacing constructors can contain"] # [doc = " false-positives. Also applying suggestions can require modification of other"] # [doc = " pieces of code, possibly including external crates."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " # use std::hash::{Hash, BuildHasher};"] # [doc = " # trait Serialize {};"] # [doc = " impl<K: Hash + Eq, V> Serialize for HashMap<K, V> { }"] # [doc = ""] # [doc = " pub fn foo(map: &mut HashMap<i32, i32>) { }"] # [doc = " ```"] # [doc = " could be rewritten as"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " # use std::hash::{Hash, BuildHasher};"] # [doc = " # trait Serialize {};"] # [doc = " impl<K: Hash + Eq, V, S: BuildHasher> Serialize for HashMap<K, V, S> { }"] # [doc = ""] # [doc = " pub fn foo<S: BuildHasher>(map: &mut HashMap<i32, i32, S>) { }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IMPLICIT_HASHER , pedantic , "missing generalization over different hashers" }
};
}
