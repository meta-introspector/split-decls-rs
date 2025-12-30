// Generated macro for places_conflict (function)
macro_rules! Depcrate_places_conflictplaces_conflict {
() => {
// Module: crate::places_conflict
// Provides: {"places_conflict"}
// Dependencies: {}
# [doc = " Helper function for checking if places conflict with a mutable borrow and deep access depth."] # [doc = " This is used to check for places conflicting outside of the borrow checking code (such as in"] # [doc = " dataflow)."] pub fn places_conflict < 'tcx > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , borrow_place : Place < 'tcx > , access_place : Place < 'tcx > , bias : PlaceConflictBias ,) -> bool { borrow_conflicts_with_place (tcx , body , borrow_place , BorrowKind :: Mut { kind : MutBorrowKind :: TwoPhaseBorrow } , access_place . as_ref () , AccessDepth :: Deep , bias ,) }
};
}
