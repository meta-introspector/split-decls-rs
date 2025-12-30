// Generated macro for Cache (struct)
macro_rules! Depcrate_nfa_thompson_pikevmCache {
() => {
// Module: crate::nfa::thompson::pikevm
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A cache represents mutable state that a [`PikeVM`] requires during a"] # [doc = " search."] # [doc = ""] # [doc = " For a given [`PikeVM`], its corresponding cache may be created either via"] # [doc = " [`PikeVM::create_cache`], or via [`Cache::new`]. They are equivalent in"] # [doc = " every way, except the former does not require explicitly importing `Cache`."] # [doc = ""] # [doc = " A particular `Cache` is coupled with the [`PikeVM`] from which it"] # [doc = " was created. It may only be used with that `PikeVM`. A cache and its"] # [doc = " allocations may be re-purposed via [`Cache::reset`], in which case, it can"] # [doc = " only be used with the new `PikeVM` (and not the old one)."] # [derive (Clone , Debug)] pub struct Cache { # [doc = " Stack used while computing epsilon closure. This effectively lets us"] # [doc = " move what is more naturally expressed through recursion to a stack"] # [doc = " on the heap."] stack : Vec < FollowEpsilon > , # [doc = " The current active states being explored for the current byte in the"] # [doc = " haystack."] curr : ActiveStates , # [doc = " The next set of states we're building that will be explored for the"] # [doc = " next byte in the haystack."] next : ActiveStates , }
};
}
