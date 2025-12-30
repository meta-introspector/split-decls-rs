// Generated macro for saturating_abs_i64 (macro)
macro_rules! Depcrate_macrossaturating_abs_i64 {
() => {
// Module: crate::macros
// Provides: {"saturating_abs_i64"}
// Dependencies: {}
macro_rules ! saturating_abs_i64 { ($ val : expr) => { if $ val . is_negative () { match $ val . checked_neg () { Some (v) => v , None => i64 :: MAX , } } else { $ val } } ; }
};
}
