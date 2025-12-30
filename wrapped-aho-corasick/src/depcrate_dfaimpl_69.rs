// Generated macro for impl_69 (impl)
macro_rules! Depcrate_dfaimpl_69 {
() => {
// Module: crate::dfa
// Provides: {"impl_69"}
// Dependencies: {}
impl DFA { # [doc = " A sentinel state ID indicating that a search should stop once it has"] # [doc = " entered this state. When a search stops, it returns a match if one has"] # [doc = " been found, otherwise no match. A DFA always has an actual dead state"] # [doc = " at this ID."] # [doc = ""] # [doc = " N.B. DFAs, unlike NFAs, do not have any notion of a FAIL state."] # [doc = " Namely, the whole point of a DFA is that the FAIL state is completely"] # [doc = " compiled away. That is, DFA construction involves pre-computing the"] # [doc = " failure transitions everywhere, such that failure transitions are no"] # [doc = " longer used at search time. This, combined with its uniformly dense"] # [doc = " representation, are the two most important factors in why it's faster"] # [doc = " than the NFAs in this crate."] const DEAD : StateID = StateID :: new_unchecked (0) ; # [doc = " Adds the given pattern IDs as matches to the given state and also"] # [doc = " records the added memory usage."] fn set_matches (& mut self , sid : StateID , pids : impl Iterator < Item = PatternID > ,) { let index = (sid . as_usize () >> self . stride2) . checked_sub (2) . unwrap () ; let mut at_least_one = false ; for pid in pids { self . matches [index] . push (pid) ; self . matches_memory_usage += PatternID :: SIZE ; at_least_one = true ; } assert ! (at_least_one , "match state must have non-empty pids") ; } }
};
}
