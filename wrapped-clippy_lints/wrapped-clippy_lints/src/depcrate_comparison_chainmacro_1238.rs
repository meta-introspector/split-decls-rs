// Generated macro for macro_1238 (macro)
macro_rules! Depcrate_comparison_chainmacro_1238 {
() => {
// Module: crate::comparison_chain
// Provides: {"macro_1238"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks comparison chains written with `if` that can be"] # [doc = " rewritten with `match` and `cmp`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `if` is not guaranteed to be exhaustive and conditionals can get"] # [doc = " repetitive"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The match statement may be slower due to the compiler"] # [doc = " not inlining the call to cmp. See issue [#5354](https://github.com/rust-lang/rust-clippy/issues/5354)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " # fn c() {}"] # [doc = " fn f(x: u8, y: u8) {"] # [doc = "     if x > y {"] # [doc = "         a()"] # [doc = "     } else if x < y {"] # [doc = "         b()"] # [doc = "     } else {"] # [doc = "         c()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " use std::cmp::Ordering;"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " # fn c() {}"] # [doc = " fn f(x: u8, y: u8) {"] # [doc = "      match x.cmp(&y) {"] # [doc = "          Ordering::Greater => a(),"] # [doc = "          Ordering::Less => b(),"] # [doc = "          Ordering::Equal => c()"] # [doc = "      }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.40.0"] pub COMPARISON_CHAIN , pedantic , "`if`s that can be rewritten with `match` and `cmp`" }
};
}
