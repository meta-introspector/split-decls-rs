// Generated macro for over (function)
macro_rules! Depcrate_hir_utilsover {
() => {
// Module: crate::hir_utils
// Provides: {"over"}
// Dependencies: {}
# [doc = " Checks if two slices are equal as per `eq_fn`."] pub fn over < X , Y > (left : & [X] , right : & [Y] , mut eq_fn : impl FnMut (& X , & Y) -> bool) -> bool { left . len () == right . len () && left . iter () . zip (right) . all (| (x , y) | eq_fn (x , y)) }
};
}
