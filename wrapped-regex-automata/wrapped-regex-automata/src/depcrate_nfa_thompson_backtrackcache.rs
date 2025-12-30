// Generated macro for Cache (struct)
macro_rules! Depcrate_nfa_thompson_backtrackCache {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A cache represents mutable state that a [`BoundedBacktracker`] requires"] # [doc = " during a search."] # [doc = ""] # [doc = " For a given [`BoundedBacktracker`], its corresponding cache may be created"] # [doc = " either via [`BoundedBacktracker::create_cache`], or via [`Cache::new`]."] # [doc = " They are equivalent in every way, except the former does not require"] # [doc = " explicitly importing `Cache`."] # [doc = ""] # [doc = " A particular `Cache` is coupled with the [`BoundedBacktracker`] from which"] # [doc = " it was created. It may only be used with that `BoundedBacktracker`. A cache"] # [doc = " and its allocations may be re-purposed via [`Cache::reset`], in which case,"] # [doc = " it can only be used with the new `BoundedBacktracker` (and not the old"] # [doc = " one)."] # [derive (Clone , Debug)] pub struct Cache { # [doc = " Stack used on the heap for doing backtracking instead of the"] # [doc = " traditional recursive approach. We don't want recursion because then"] # [doc = " we're likely to hit a stack overflow for bigger regexes."] stack : Vec < Frame > , # [doc = " The set of (StateID, HaystackOffset) pairs that have been visited"] # [doc = " by the backtracker within a single search. If such a pair has been"] # [doc = " visited, then we avoid doing the work for that pair again. This is"] # [doc = " what \"bounds\" the backtracking and prevents it from having worst case"] # [doc = " exponential time."] visited : Visited , }
};
}
