// Generated macro for restrict_precision_for_unsafe (function)
macro_rules! Depcrate_upvarrestrict_precision_for_unsafe {
() => {
// Module: crate::upvar
// Provides: {"restrict_precision_for_unsafe"}
// Dependencies: {}
# [doc = " Truncate `place` so that an `unsafe` block isn't required to capture it."] # [doc = " - No projections are applied to raw pointers, since these require unsafe blocks. We capture"] # [doc = "   them completely."] # [doc = " - No projections are applied on top of Union ADTs, since these require unsafe blocks."] fn restrict_precision_for_unsafe (mut place : Place < '_ > , mut curr_mode : ty :: UpvarCapture ,) -> (Place < '_ > , ty :: UpvarCapture) { if place . base_ty . is_raw_ptr () { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , 0) ; } if place . base_ty . is_union () { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , 0) ; } for (i , proj) in place . projections . iter () . enumerate () { if proj . ty . is_raw_ptr () { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , i + 1) ; break ; } if proj . ty . is_union () { truncate_place_to_len_and_update_capture_kind (& mut place , & mut curr_mode , i + 1) ; break ; } } (place , curr_mode) }
};
}
