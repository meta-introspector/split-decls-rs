// Generated macro for both_some_and (function)
macro_rules! Depcrate_hir_utilsboth_some_and {
() => {
// Module: crate::hir_utils
// Provides: {"both_some_and"}
// Dependencies: {}
# [doc = " Checks if the two `Option`s are both `Some` and pass the predicate function."] pub fn both_some_and < X , Y > (l : Option < X > , r : Option < Y > , mut pred : impl FnMut (X , Y) -> bool) -> bool { l . is_some_and (| l | r . is_some_and (| r | pred (l , r))) }
};
}
