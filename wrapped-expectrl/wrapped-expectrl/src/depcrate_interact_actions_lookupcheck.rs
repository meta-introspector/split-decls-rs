// Generated macro for check (function)
macro_rules! Depcrate_interact_actions_lookupcheck {
() => {
// Module: crate::interact::actions::lookup
// Provides: {"check"}
// Dependencies: {}
fn check < N > (buf : & mut Vec < u8 > , needle : N , eof : bool) -> Result < Option < Captures > , Error > where N : Needle , { let found = needle . check (buf , eof) ? ; if found . is_empty () { return Ok (None) ; } let end_index = Captures :: right_most_index (& found) ; let involved_bytes = buf [.. end_index] . to_vec () ; let found = Captures :: new (involved_bytes , found) ; let _ = buf . drain (.. end_index) ; Ok (Some (found)) }
};
}
