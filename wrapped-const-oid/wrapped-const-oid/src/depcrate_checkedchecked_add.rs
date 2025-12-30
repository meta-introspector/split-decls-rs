// Generated macro for checked_add (macro)
macro_rules! Depcrate_checkedchecked_add {
() => {
// Module: crate::checked
// Provides: {"checked_add"}
// Dependencies: {}
# [doc = " `const fn`-friendly checked addition helper."] macro_rules ! checked_add { ($ a : expr , $ b : expr) => { match $ a . checked_add ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
};
}
