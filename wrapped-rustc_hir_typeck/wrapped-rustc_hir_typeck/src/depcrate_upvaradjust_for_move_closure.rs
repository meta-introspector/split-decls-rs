// Generated macro for adjust_for_move_closure (function)
macro_rules! Depcrate_upvaradjust_for_move_closure {
() => {
// Module: crate::upvar
// Provides: {"adjust_for_move_closure"}
// Dependencies: {}
# [doc = " Truncate deref of any reference."] fn adjust_for_move_closure (mut place : Place < '_ > , mut kind : ty :: UpvarCapture ,) -> (Place < '_ > , ty :: UpvarCapture) { let first_deref = place . projections . iter () . position (| proj | proj . kind == ProjectionKind :: Deref) ; if let Some (idx) = first_deref { truncate_place_to_len_and_update_capture_kind (& mut place , & mut kind , idx) ; } (place , ty :: UpvarCapture :: ByValue) }
};
}
