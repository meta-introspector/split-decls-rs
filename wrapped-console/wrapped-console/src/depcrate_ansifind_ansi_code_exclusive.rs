// Generated macro for find_ansi_code_exclusive (function)
macro_rules! Depcrate_ansifind_ansi_code_exclusive {
() => {
// Module: crate::ansi
// Provides: {"find_ansi_code_exclusive"}
// Dependencies: {}
fn find_ansi_code_exclusive (it : & mut Peekable < CharIndices >) -> Option < (usize , usize) > { 'outer : loop { if let (start , '\u{1b}') | (start , '\u{9b}') = it . peek () ? { let start = * start ; let mut state = State :: default () ; let mut maybe_end = None ; loop { let item = it . peek () ; if let Some ((idx , c)) = item { state . transition (* c) ; if state . is_final () { maybe_end = Some (* idx) ; } } if state . is_trapped () || item . is_none () { match maybe_end { Some (end) => { return Some ((start , end + 1)) ; } None => continue 'outer , } } it . next () ; } } it . next () ; } }
};
}
