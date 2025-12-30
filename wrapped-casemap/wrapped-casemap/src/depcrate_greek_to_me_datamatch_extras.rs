// Generated macro for match_extras (function)
macro_rules! Depcrate_greek_to_me_datamatch_extras {
() => {
// Module: crate::greek_to_me::data
// Provides: {"match_extras"}
// Dependencies: {}
# [doc = " Characters like the ohm sign that do not belong in the two blocks above"] pub (crate) fn match_extras (ch : char) -> Option < u8 > { Some (match ch { 'Ω' => 7 , _ => return None }) }
};
}
