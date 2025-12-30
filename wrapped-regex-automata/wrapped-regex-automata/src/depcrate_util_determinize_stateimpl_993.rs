// Generated macro for impl_993 (impl)
macro_rules! Depcrate_util_determinize_stateimpl_993 {
() => {
// Module: crate::util::determinize::state
// Provides: {"impl_993"}
// Dependencies: {}
# [doc = " For docs on these routines, see the internal Repr and ReprVec types below."] impl StateBuilderMatches { pub (crate) fn into_nfa (mut self) -> StateBuilderNFA { self . repr_vec () . close_match_pattern_ids () ; StateBuilderNFA { repr : self . 0 , prev_nfa_state_id : StateID :: ZERO } } pub (crate) fn set_is_from_word (& mut self) { self . repr_vec () . set_is_from_word () } pub (crate) fn set_is_half_crlf (& mut self) { self . repr_vec () . set_is_half_crlf () } pub (crate) fn look_have (& self) -> LookSet { LookSet :: read_repr (& self . 0 [1 ..]) } pub (crate) fn set_look_have (& mut self , set : impl FnMut (LookSet) -> LookSet ,) { self . repr_vec () . set_look_have (set) } pub (crate) fn add_match_pattern_id (& mut self , pid : PatternID) { self . repr_vec () . add_match_pattern_id (pid) } fn repr (& self) -> Repr < '_ > { Repr (& self . 0) } fn repr_vec (& mut self) -> ReprVec < '_ > { ReprVec (& mut self . 0) } }
};
}
