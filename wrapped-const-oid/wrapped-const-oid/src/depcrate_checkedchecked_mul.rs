// Generated macro for checked_mul (macro)
macro_rules! Depcrate_checkedchecked_mul {
() => {
// Module: crate::checked
// Provides: {"checked_mul"}
// Dependencies: {}
# [doc = " `const fn`-friendly checked multiplication helper."] macro_rules ! checked_mul { ($ a : expr , $ b : expr) => { match $ a . checked_mul ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
};
}
