// Generated macro for macro_7164 (macro)
macro_rules! Depcrate_methodsmacro_7164 {
() => {
// Module: crate::methods
// Provides: {"macro_7164"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `from_iter()` function calls on types that implement the `FromIterator`"] # [doc = " trait."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If it's needed to create a collection from the contents of an iterator, the `Iterator::collect(_)`"] # [doc = " method is preferred. However, when it's needed to specify the container type,"] # [doc = " `Vec::from_iter(_)` can be more readable than using a turbofish (e.g. `_.collect::<Vec<_>>()`). See"] # [doc = " [FromIterator documentation](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let five_fives = std::iter::repeat(5).take(5);"] # [doc = ""] # [doc = " let v = Vec::from_iter(five_fives);"] # [doc = ""] # [doc = " assert_eq!(v, vec![5, 5, 5, 5, 5]);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let five_fives = std::iter::repeat(5).take(5);"] # [doc = ""] # [doc = " let v: Vec<i32> = five_fives.collect();"] # [doc = ""] # [doc = " assert_eq!(v, vec![5, 5, 5, 5, 5]);"] # [doc = " ```"] # [doc = " but prefer to use"] # [doc = " ```no_run"] # [doc = " let numbers: Vec<i32> = FromIterator::from_iter(1..=5);"] # [doc = " ```"] # [doc = " instead of"] # [doc = " ```no_run"] # [doc = " let numbers = (1..=5).collect::<Vec<_>>();"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub FROM_ITER_INSTEAD_OF_COLLECT , pedantic , "use `.collect()` instead of `::from_iter()`" }
};
}
