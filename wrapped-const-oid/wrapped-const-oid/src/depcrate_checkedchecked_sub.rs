// Generated macro for checked_sub (macro)
macro_rules! Depcrate_checkedchecked_sub {
() => {
// Module: crate::checked
// Provides: {"checked_sub"}
// Dependencies: {}
# [doc = " `const fn`-friendly checked subtraction helper."] macro_rules ! checked_sub { ($ a : expr , $ b : expr) => { match $ a . checked_sub ($ b) { Some (n) => n , None => return Err (Error :: Overflow) , } } ; }
};
}
