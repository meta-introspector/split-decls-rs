// Generated macro for NextInsert (struct)
macro_rules! Depcrate_nfa_thompson_range_trieNextInsert {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"NextInsert"}
// Dependencies: {}
# [doc = " The next state to process during insertion and any remaining ranges that we"] # [doc = " want to add for a particular sequence of ranges. The first such instance"] # [doc = " is always the root state along with all ranges given."] # [derive (Clone , Debug)] struct NextInsert { # [doc = " The next state to begin inserting ranges. This state should be the"] # [doc = " state at which `ranges[0]` should be inserted."] state_id : StateID , # [doc = " The ranges to insert. We used a fixed-size array here to avoid an"] # [doc = " allocation."] ranges : [Utf8Range ; 4] , # [doc = " The number of valid ranges in the above array."] len : u8 , }
};
}
