// Generated macro for repeat (function)
macro_rules! Depcrate_iter_repeatrepeat {
() => {
// Module: crate::iter::repeat
// Provides: {"repeat"}
// Dependencies: {}
# [doc = " Creates a parallel iterator that endlessly repeats `element` (by"] # [doc = " cloning it). Note that this iterator has \"infinite\" length, so"] # [doc = " typically you would want to use `zip` or `take` or some other"] # [doc = " means to shorten it, or consider using"] # [doc = " [the `repeat_n()` function] instead."] # [doc = ""] # [doc = " [the `repeat_n()` function]: repeat_n()"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = " use rayon::iter::repeat;"] # [doc = " let x: Vec<(i32, i32)> = repeat(22).zip(0..3).collect();"] # [doc = " assert_eq!(x, vec![(22, 0), (22, 1), (22, 2)]);"] # [doc = " ```"] pub fn repeat < T : Clone + Send > (element : T) -> Repeat < T > { Repeat { element } }
};
}
