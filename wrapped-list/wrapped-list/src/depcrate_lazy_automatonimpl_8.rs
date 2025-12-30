// Generated macro for impl_8 (impl)
macro_rules! Depcrate_lazy_automatonimpl_8 {
() => {
// Module: crate::lazy_automaton
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : AsRef < [u8] > > LazyAutomaton for DFA < T > { fn matches_earliest_fwd_lazy < S : Writeable + ? Sized > (& self , haystack : & S) -> bool { struct DFAStepper < 'a > { dfa : & 'a DFA < & 'a [u8] > , state : StateID , } impl core :: fmt :: Write for DFAStepper < '_ > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { for & byte in s . as_bytes () { self . state = self . dfa . next_state (self . state , byte) ; if self . dfa . is_match_state (self . state) || self . dfa . is_dead_state (self . state) { return Err (core :: fmt :: Error) ; } } Ok (()) } } let Ok (start_state) = self . start_state (& StartConfig :: new () . anchored (regex_automata :: Anchored :: Yes)) else { return false ; } ; let mut stepper = DFAStepper { state : start_state , dfa : & self . as_ref () , } ; if haystack . write_to (& mut stepper) . is_ok () { stepper . state = self . next_eoi_state (stepper . state) ; } self . is_match_state (stepper . state) } }
};
}
