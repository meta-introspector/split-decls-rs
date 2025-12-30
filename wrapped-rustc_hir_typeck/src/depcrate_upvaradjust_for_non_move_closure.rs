// Generated macro for adjust_for_non_move_closure (function)
macro_rules! Depcrate_upvaradjust_for_non_move_closure {
() => {
// Module: crate::upvar
// Provides: {"adjust_for_non_move_closure"}
// Dependencies: {}
# [doc = " Adjust closure capture just that if taking ownership of data, only move data"] # [doc = " from enclosing stack frame."] fn adjust_for_non_move_closure (mut place : Place < '_ > , mut kind : ty :: UpvarCapture ,) -> (Place < '_ > , ty :: UpvarCapture) { let contains_deref = place . projections . iter () . position (| proj | proj . kind == ProjectionKind :: Deref) ; match kind { ty :: UpvarCapture :: ByValue | ty :: UpvarCapture :: ByUse => { if let Some (idx) = contains_deref { truncate_place_to_len_and_update_capture_kind (& mut place , & mut kind , idx) ; } } ty :: UpvarCapture :: ByRef (..) => { } } (place , kind) }
};
}
