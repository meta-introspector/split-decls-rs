// Generated macro for NextDupe (struct)
macro_rules! Depcrate_nfa_thompson_range_trieNextDupe {
() => {
// Module: crate::nfa::thompson::range_trie
// Provides: {"NextDupe"}
// Dependencies: {}
# [doc = " The next state to process during duplication."] # [derive (Clone , Debug)] struct NextDupe { # [doc = " The state we want to duplicate."] old_id : StateID , # [doc = " The ID of the new state that is a duplicate of old_id."] new_id : StateID , }
};
}
