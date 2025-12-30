// Generated macro for restrict_capture_precision (function)
macro_rules! Depcrate_upvarrestrict_capture_precision {
() => {
// Module: crate::upvar
// Provides: {"restrict_capture_precision"}
// Dependencies: {}
# [doc = " Truncate projections so that following rules are obeyed by the captured `place`:"] # [doc = " - No Index projections are captured, since arrays are captured completely."] # [doc = " - No unsafe block is required to capture `place`"] # [doc = " Returns the truncated place and updated capture mode."] fn restrict_capture_precision (place : Place < '_ > , curr_mode : ty :: UpvarCapture ,) -> (Place < '_ > , ty :: UpvarCapture) { let (mut place , mut curr_mode) = restrict_precision_for_unsafe (place , curr_mode) ; if place . projections . is_empty () { return (place , curr_mode) ; } for (i , proj) in place . projections . iter () . enumerate () { match proj . kind { ProjectionKind :: Index | ProjectionKind :: Subslice => { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , i) ; return (place , curr_mode) ; } ProjectionKind :: Deref => { } ProjectionKind :: OpaqueCast => { } ProjectionKind :: Field (..) => { } ProjectionKind :: UnwrapUnsafeBinder => { } } } (place , curr_mode) }
};
}
