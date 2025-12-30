// Generated macro for count_eq (function)
macro_rules! Depcrate_hir_utilscount_eq {
() => {
// Module: crate::hir_utils
// Provides: {"count_eq"}
// Dependencies: {}
# [doc = " Counts how many elements of the slices are equal as per `eq_fn`."] pub fn count_eq < X : Sized > (left : & mut dyn Iterator < Item = X > , right : & mut dyn Iterator < Item = X > , mut eq_fn : impl FnMut (& X , & X) -> bool ,) -> usize { left . zip (right) . take_while (| (l , r) | eq_fn (l , r)) . count () }
};
}
