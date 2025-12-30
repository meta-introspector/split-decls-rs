// Generated macro for assert_eq_warn (macro)
macro_rules! Depcrate_utilsassert_eq_warn {
() => {
// Module: crate::utils
// Provides: {"assert_eq_warn"}
// Dependencies: {}
macro_rules ! assert_eq_warn { ($ x : expr , $ y : expr) => { { let x = $ x ; let y = $ y ; if x != y { eprintln ! ("assert failed: {:?}: {:?} != {:?}: {:?}" , stringify ! ($ x) , x , stringify ! ($ y) , y) ; } } } ; }
};
}
