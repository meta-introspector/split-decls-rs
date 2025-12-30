// Generated macro for iterate (function)
macro_rules! Depcrate_sourcesiterate {
() => {
// Module: crate::sources
// Provides: {"iterate"}
// Dependencies: {}
# [doc = " Creates a new iterator that infinitely applies function to value and yields results."] # [doc = ""] # [doc = " ```"] # [doc = " use itertools::iterate;"] # [doc = ""] # [doc = " itertools::assert_equal(iterate(1, |i| i % 3 + 1).take(5), vec![1, 2, 3, 1, 2]);"] # [doc = " ```"] # [doc = ""] # [doc = " **Panics** if compute the next value does."] # [doc = ""] # [doc = " ```should_panic"] # [doc = " # use itertools::iterate;"] # [doc = " let mut it = iterate(25u32, |x| x - 10).take_while(|&x| x > 10);"] # [doc = " assert_eq!(it.next(), Some(25)); // `Iterate` holds 15."] # [doc = " assert_eq!(it.next(), Some(15)); // `Iterate` holds 5."] # [doc = " it.next(); // `5 - 10` overflows."] # [doc = " ```"] # [doc = ""] # [doc = " You can alternatively use [`core::iter::successors`] as it better describes a finite iterator."] pub fn iterate < St , F > (initial_value : St , f : F) -> Iterate < St , F > where F : FnMut (& St) -> St , { Iterate { state : initial_value , f , } }
};
}
