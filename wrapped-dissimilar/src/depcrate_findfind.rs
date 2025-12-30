// Generated macro for find (function)
macro_rules! Depcrate_findfind {
() => {
// Module: crate::find
// Provides: {"find"}
// Dependencies: {}
pub fn find (haystack : & [char] , needle : & [char]) -> Option < usize > { assert ! (! needle . is_empty ()) ; let (crit_pos_false , period_false) = maximal_suffix (needle , false) ; let (crit_pos_true , period_true) = maximal_suffix (needle , true) ; let (crit_pos , mut period) = if crit_pos_false > crit_pos_true { (crit_pos_false , period_false) } else { (crit_pos_true , period_true) } ; let byteset ; let mut memory ; let long_period = needle [.. crit_pos] != needle [period .. period + crit_pos] ; if long_period { period = cmp :: max (crit_pos , needle . len () - crit_pos) + 1 ; byteset = byteset_create (needle) ; memory = usize :: MAX ; } else { byteset = byteset_create (& needle [.. period]) ; memory = 0 ; } let mut position = 0 ; let needle_last = needle . len () - 1 ; 'search : loop { let tail_byte = * haystack . get (position + needle_last) ? ; if ! byteset_contains (byteset , tail_byte) { position += needle . len () ; if ! long_period { memory = 0 ; } continue 'search ; } let start = if long_period { crit_pos } else { cmp :: max (crit_pos , memory) } ; for i in start .. needle . len () { if needle [i] != haystack [position + i] { position += i - crit_pos + 1 ; if ! long_period { memory = 0 ; } continue 'search ; } } let start = if long_period { 0 } else { memory } ; for i in (start .. crit_pos) . rev () { if needle [i] != haystack [position + i] { position += period ; if ! long_period { memory = needle . len () - period ; } continue 'search ; } } return Some (position) ; } }
};
}
