// Generated macro for empty (function)
macro_rules! Depcrate_iter_emptyempty {
() => {
// Module: crate::iter::empty
// Provides: {"empty"}
// Dependencies: {}
# [doc = " Creates a parallel iterator that produces nothing."] # [doc = ""] # [doc = " This admits no parallelism on its own, but it could be used for code that"] # [doc = " deals with generic parallel iterators."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = " use rayon::iter::empty;"] # [doc = ""] # [doc = " let pi = (0..1234).into_par_iter()"] # [doc = "     .chain(empty())"] # [doc = "     .chain(1234..10_000);"] # [doc = ""] # [doc = " assert_eq!(pi.count(), 10_000);"] # [doc = " ```"] pub fn empty < T : Send > () -> Empty < T > { Empty { marker : PhantomData , } }
};
}
