// Generated macro for fmt_state_indicator (function)
macro_rules! Depcrate_dfa_automatonfmt_state_indicator {
() => {
// Module: crate::dfa::automaton
// Provides: {"fmt_state_indicator"}
// Dependencies: {}
# [doc = " Write a prefix \"state\" indicator for fmt::Debug impls."] # [doc = ""] # [doc = " Specifically, this tries to succinctly distinguish the different types of"] # [doc = " states: dead states, quit states, accelerated states, start states and"] # [doc = " match states. It even accounts for the possible overlappings of different"] # [doc = " state types."] pub (crate) fn fmt_state_indicator < A : Automaton > (f : & mut core :: fmt :: Formatter < '_ > , dfa : A , id : StateID ,) -> core :: fmt :: Result { if dfa . is_dead_state (id) { write ! (f , "D") ? ; if dfa . is_start_state (id) { write ! (f , ">") ? ; } else { write ! (f , " ") ? ; } } else if dfa . is_quit_state (id) { write ! (f , "Q ") ? ; } else if dfa . is_start_state (id) { if dfa . is_accel_state (id) { write ! (f , "A>") ? ; } else { write ! (f , " >") ? ; } } else if dfa . is_match_state (id) { if dfa . is_accel_state (id) { write ! (f , "A*") ? ; } else { write ! (f , " *") ? ; } } else if dfa . is_accel_state (id) { write ! (f , "A ") ? ; } else { write ! (f , "  ") ? ; } Ok (()) }
};
}
