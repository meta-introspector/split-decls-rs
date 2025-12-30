// Generated macro for get_match (function)
macro_rules! Depcrate_automatonget_match {
() => {
// Module: crate::automaton
// Provides: {"get_match"}
// Dependencies: {}
# [inline (always)] fn get_match < A : Automaton + ? Sized > (aut : & A , sid : StateID , index : usize , at : usize ,) -> Match { let pid = aut . match_pattern (sid , index) ; let len = aut . pattern_len (pid) ; Match :: new (pid , (at - len) .. at) }
};
}
