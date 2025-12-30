// Generated macro for in_parallel_if (function)
macro_rules! Depcrate_parallelin_parallel_if {
() => {
// Module: crate::parallel
// Provides: {"in_parallel_if"}
// Dependencies: {}
# [doc = " Run [`in_parallel()`] only if the given `condition()` returns true when eagerly evaluated."] # [doc = ""] # [doc = " For parameters, see the documentation of [`in_parallel()`]"] # [doc = ""] # [doc = " Note that the non-parallel version is equivalent to [`in_parallel()`]."] # [cfg (not (feature = "parallel"))] pub fn in_parallel_if < I , S , O , R > (_condition : impl FnOnce () -> bool , input : impl Iterator < Item = I > , thread_limit : Option < usize > , new_thread_state : impl FnOnce (usize) -> S , consume : impl FnMut (I , & mut S) -> O , reducer : R ,) -> Result < < R as Reduce > :: Output , < R as Reduce > :: Error > where R : Reduce < Input = O > , I : Send , O : Send , { serial :: in_parallel (input , thread_limit , new_thread_state , consume , reducer) }
};
}
