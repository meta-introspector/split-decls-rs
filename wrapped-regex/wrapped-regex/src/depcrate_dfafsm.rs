// Generated macro for Fsm (struct)
macro_rules! Depcrate_dfaFsm {
() => {
// Module: crate::dfa
// Provides: {"Fsm"}
// Dependencies: {}
# [doc = " Fsm encapsulates the actual execution of the DFA."] # [derive (Debug)] pub struct Fsm < 'a > { # [doc = " prog contains the NFA instruction opcodes. DFA execution uses either"] # [doc = " the `dfa` instructions or the `dfa_reverse` instructions from"] # [doc = " `exec::ExecReadOnly`. (It never uses `ExecReadOnly.nfa`, which may have"] # [doc = " Unicode opcodes that cannot be executed by the DFA.)"] prog : & 'a Program , # [doc = " The start state. We record it here because the pointer may change"] # [doc = " when the cache is wiped."] start : StatePtr , # [doc = " The current position in the input."] at : usize , # [doc = " Should we quit after seeing the first match? e.g., When the caller"] # [doc = " uses `is_match` or `shortest_match`."] quit_after_match : bool , # [doc = " The last state that matched."] # [doc = ""] # [doc = " When no match has occurred, this is set to STATE_UNKNOWN."] # [doc = ""] # [doc = " This is only useful when matching regex sets. The last match state"] # [doc = " is useful because it contains all of the match instructions seen,"] # [doc = " thereby allowing us to enumerate which regexes in the set matched."] last_match_si : StatePtr , # [doc = " The input position of the last cache flush. We use this to determine"] # [doc = " if we're thrashing in the cache too often. If so, the DFA quits so"] # [doc = " that we can fall back to the NFA algorithm."] last_cache_flush : usize , # [doc = " All cached DFA information that is persisted between searches."] cache : & 'a mut CacheInner , }
};
}
