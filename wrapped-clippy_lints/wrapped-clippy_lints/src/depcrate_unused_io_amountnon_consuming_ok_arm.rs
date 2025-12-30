// Generated macro for non_consuming_ok_arm (function)
macro_rules! Depcrate_unused_io_amountnon_consuming_ok_arm {
() => {
// Module: crate::unused_io_amount
// Provides: {"non_consuming_ok_arm"}
// Dependencies: {}
fn non_consuming_ok_arm < 'a > (cx : & LateContext < 'a > , arm : & hir :: Arm < 'a >) -> bool { if arm . guard . is_some () { return false ; } if is_unreachable_or_panic (cx , arm . body) { return false ; } if is_ok_wild_or_dotdot_pattern (cx , arm . pat) { return true ; } false }
};
}
