// Generated macro for macro_7069 (macro)
macro_rules! Depcrate_methodsmacro_7069 {
() => {
// Module: crate::methods
// Provides: {"macro_7069"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `bool::then` in `Iterator::filter_map`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written with `filter` then `map` instead, which would reduce nesting and"] # [doc = " separates the filtering from the transformation phase. This comes with no cost to"] # [doc = " performance and is just cleaner."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " Does not lint `bool::then_some`, as it eagerly evaluates its arguments rather than lazily."] # [doc = " This can create differing behavior, so better safe than sorry."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn really_expensive_fn(i: i32) -> i32 { i }"] # [doc = " # let v = vec![];"] # [doc = " _ = v.into_iter().filter_map(|i| (i % 2 == 0).then(|| really_expensive_fn(i)));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn really_expensive_fn(i: i32) -> i32 { i }"] # [doc = " # let v = vec![];"] # [doc = " _ = v.into_iter().filter(|i| i % 2 == 0).map(|i| really_expensive_fn(i));"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub FILTER_MAP_BOOL_THEN , style , "checks for usage of `bool::then` in `Iterator::filter_map`" }
};
}
