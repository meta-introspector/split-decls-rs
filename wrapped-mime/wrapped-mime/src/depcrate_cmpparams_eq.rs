// Generated macro for params_eq (function)
macro_rules! Depcrate_cmpparams_eq {
() => {
// Module: crate::cmp
// Provides: {"params_eq"}
// Dependencies: {}
fn params_eq (a : & Mime , b : & Mime) -> bool { if a . params () . size_hint () != b . params () . size_hint () { return false ; } for (name , value) in crate :: value :: params (a) { if crate :: value :: param (b , name) != Some (value) { return false ; } } true }
};
}
