// Generated macro for saturating_abs_i32 (macro)
macro_rules! Depcrate_macrossaturating_abs_i32 {
() => {
// Module: crate::macros
// Provides: {"saturating_abs_i32"}
// Dependencies: {}
macro_rules ! saturating_abs_i32 { ($ val : expr) => { if $ val . is_negative () { match $ val . checked_neg () { Some (v) => v , None => i32 :: MAX , } } else { $ val } } ; }
};
}
