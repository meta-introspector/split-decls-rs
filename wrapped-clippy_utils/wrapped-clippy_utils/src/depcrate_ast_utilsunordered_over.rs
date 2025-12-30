// Generated macro for unordered_over (function)
macro_rules! Depcrate_ast_utilsunordered_over {
() => {
// Module: crate::ast_utils
// Provides: {"unordered_over"}
// Dependencies: {}
# [doc = " Checks if each element in the first slice is contained within the latter as per `eq_fn`."] pub fn unordered_over < X , Y > (left : & [X] , right : & [Y] , mut eq_fn : impl FnMut (& X , & Y) -> bool) -> bool { left . len () == right . len () && left . iter () . all (| l | right . iter () . any (| r | eq_fn (l , r))) }
};
}
