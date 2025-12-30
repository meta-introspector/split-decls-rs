// Generated macro for once (function)
macro_rules! Depcrate_iter_onceonce {
() => {
// Module: crate::iter::once
// Provides: {"once"}
// Dependencies: {}
# [doc = " Creates a parallel iterator that produces an element exactly once."] # [doc = ""] # [doc = " This admits no parallelism on its own, but it could be chained to existing"] # [doc = " parallel iterators to extend their contents, or otherwise used for any code"] # [doc = " that deals with generic parallel iterators."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = " use rayon::iter::once;"] # [doc = ""] # [doc = " let pi = (0..1234).into_par_iter()"] # [doc = "     .chain(once(-1))"] # [doc = "     .chain(1234..10_000);"] # [doc = ""] # [doc = " assert_eq!(pi.clone().count(), 10_001);"] # [doc = " assert_eq!(pi.clone().filter(|&x| x < 0).count(), 1);"] # [doc = " assert_eq!(pi.position_any(|x| x < 0), Some(1234));"] # [doc = " ```"] pub fn once < T : Send > (item : T) -> Once < T > { Once { item } }
};
}
