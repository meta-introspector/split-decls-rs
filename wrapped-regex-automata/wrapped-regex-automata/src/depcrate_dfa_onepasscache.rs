// Generated macro for Cache (struct)
macro_rules! Depcrate_dfa_onepassCache {
() => {
// Module: crate::dfa::onepass
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " A cache represents mutable state that a one-pass [`DFA`] requires during a"] # [doc = " search."] # [doc = ""] # [doc = " For a given one-pass DFA, its corresponding cache may be created either via"] # [doc = " [`DFA::create_cache`], or via [`Cache::new`]. They are equivalent in every"] # [doc = " way, except the former does not require explicitly importing `Cache`."] # [doc = ""] # [doc = " A particular `Cache` is coupled with the one-pass DFA from which it was"] # [doc = " created. It may only be used with that one-pass DFA. A cache and its"] # [doc = " allocations may be re-purposed via [`Cache::reset`], in which case, it can"] # [doc = " only be used with the new one-pass DFA (and not the old one)."] # [derive (Clone , Debug)] pub struct Cache { # [doc = " Scratch space used to store slots during a search. Basically, we use"] # [doc = " the caller provided slots to store slots known when a match occurs."] # [doc = " But after a match occurs, we might continue a search but ultimately"] # [doc = " fail to extend the match. When continuing the search, we need some"] # [doc = " place to store candidate capture offsets without overwriting the slot"] # [doc = " offsets recorded for the most recently seen match."] explicit_slots : Vec < Option < NonMaxUsize > > , # [doc = " The number of slots in the caller-provided 'Captures' value for the"] # [doc = " current search. This is always at most 'explicit_slots.len()', but"] # [doc = " might be less than it, if the caller provided fewer slots to fill."] explicit_slot_len : usize , }
};
}
