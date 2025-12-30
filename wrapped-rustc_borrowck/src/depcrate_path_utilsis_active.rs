// Generated macro for is_active (function)
macro_rules! Depcrate_path_utilsis_active {
() => {
// Module: crate::path_utils
// Provides: {"is_active"}
// Dependencies: {}
pub (super) fn is_active < 'tcx > (dominators : & Dominators < BasicBlock > , borrow_data : & BorrowData < 'tcx > , location : Location ,) -> bool { debug ! ("is_active(borrow_data={:?}, location={:?})" , borrow_data , location) ; let activation_location = match borrow_data . activation_location { TwoPhaseActivation :: NotTwoPhase => return true , TwoPhaseActivation :: NotActivated => return false , TwoPhaseActivation :: ActivatedAt (loc) => loc , } ; if activation_location . dominates (location , dominators) { return true ; } let reserve_location = borrow_data . reserve_location . successor_within_block () ; if reserve_location . dominates (location , dominators) { false } else { true } }
};
}
