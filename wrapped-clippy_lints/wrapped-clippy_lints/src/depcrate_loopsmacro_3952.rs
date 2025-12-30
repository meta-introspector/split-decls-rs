// Generated macro for macro_3952 (macro)
macro_rules! Depcrate_loopsmacro_3952 {
() => {
// Module: crate::loops
// Provides: {"macro_3952"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for loops with a range bound that is a mutable variable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " One might think that modifying the mutable variable changes the loop bounds. It doesn't."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " False positive when mutation is followed by a `break`, but the `break` is not immediately"] # [doc = " after the mutation:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let mut x = 5;"] # [doc = " for _ in 0..x {"] # [doc = "     x += 1; // x is a range bound that is mutated"] # [doc = "     ..; // some other expression"] # [doc = "     break; // leaves the loop, so mutation is not an issue"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " False positive on nested loops ([#6072](https://github.com/rust-lang/rust-clippy/issues/6072))"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut foo = 42;"] # [doc = " for i in 0..foo {"] # [doc = "     foo -= 1;"] # [doc = "     println!(\"{i}\"); // prints numbers from 0 to 41, not 0 to 21"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUT_RANGE_BOUND , suspicious , "for loop over a range where one of the bounds is a mutable variable" }
};
}
