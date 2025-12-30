// Generated macro for truncate_place_to_len_and_update_capture_kind (function)
macro_rules! Depcrate_upvartruncate_place_to_len_and_update_capture_kind {
() => {
// Module: crate::upvar
// Provides: {"truncate_place_to_len_and_update_capture_kind"}
// Dependencies: {}
# [doc = " Truncates `place` to have up to `len` projections."] # [doc = " `curr_mode` is the current required capture kind for the place."] # [doc = " Returns the truncated `place` and the updated required capture kind."] # [doc = ""] # [doc = " Note: Capture kind changes from `MutBorrow` to `UniqueImmBorrow` if the truncated part of the `place`"] # [doc = " contained `Deref` of `&mut`."] fn truncate_place_to_len_and_update_capture_kind < 'tcx > (place : & mut Place < 'tcx > , curr_mode : & mut ty :: UpvarCapture , len : usize ,) { let is_mut_ref = | ty : Ty < '_ > | matches ! (ty . kind () , ty :: Ref (.., hir :: Mutability :: Mut)) ; match curr_mode { ty :: UpvarCapture :: ByRef (ty :: BorrowKind :: Mutable) => { for i in len .. place . projections . len () { if place . projections [i] . kind == ProjectionKind :: Deref && is_mut_ref (place . ty_before_projection (i)) { * curr_mode = ty :: UpvarCapture :: ByRef (ty :: BorrowKind :: UniqueImmutable) ; break ; } } } ty :: UpvarCapture :: ByRef (..) => { } ty :: UpvarCapture :: ByValue | ty :: UpvarCapture :: ByUse => { } } place . projections . truncate (len) ; }
};
}
