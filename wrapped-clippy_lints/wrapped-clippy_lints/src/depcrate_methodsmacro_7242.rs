// Generated macro for macro_7242 (macro)
macro_rules! Depcrate_methodsmacro_7242 {
() => {
// Module: crate::methods
// Provides: {"macro_7242"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for `Iterator::map` over ranges without using the parameter which"] # [doc = " could be more clearly expressed using `std::iter::repeat(...).take(...)`"] # [doc = " or `std::iter::repeat_n`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " It expresses the intent more clearly to `take` the correct number of times"] # [doc = " from a generating function than to apply a closure to each number in a"] # [doc = " range only to discard them."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let random_numbers : Vec<_> = (0..10).map(|_| { 3 + 1 }).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let f : Vec<_> = std::iter::repeat( 3 + 1 ).take(10).collect();"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known Issues"] # [doc = ""] # [doc = " This lint may suggest replacing a `Map<Range>` with a `Take<RepeatWith>`."] # [doc = " The former implements some traits that the latter does not, such as"] # [doc = " `DoubleEndedIterator`."] # [clippy :: version = "1.84.0"] pub MAP_WITH_UNUSED_ARGUMENT_OVER_RANGES , restriction , "map of a trivial closure (not dependent on parameter) over a range" }
};
}
