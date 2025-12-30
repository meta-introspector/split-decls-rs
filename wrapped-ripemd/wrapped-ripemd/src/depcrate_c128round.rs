// Generated macro for round (macro)
macro_rules! Depcrate_c128round {
() => {
// Module: crate::c128
// Provides: {"round"}
// Dependencies: {}
macro_rules ! round (($ a : expr , $ b : expr , $ c : expr , $ d : expr , $ x : expr , $ bits : expr , $ add : expr , $ round : expr) => ({ $ a = $ a . wrapping_add ($ round) . wrapping_add ($ x) . wrapping_add ($ add) ; $ a = $ a . rotate_left ($ bits) ; }) ;) ;
};
}
