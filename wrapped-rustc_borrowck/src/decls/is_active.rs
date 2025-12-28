macro_rules! deps {
    () => {
        BorrowData!();
        TwoPhaseActivation!();
    };
}

macro_rules! is_active {
    () => {
        deps!();
        pub (super) fn is_active < 'tcx > (dominators : & Dominators < BasicBlock > , borrow_data : & BorrowData < 'tcx > , location : Location ,) -> bool { debug ! ("is_active(borrow_data={:?}, location={:?})" , borrow_data , location) ; let activation_location = match borrow_data . activation_location { TwoPhaseActivation :: NotTwoPhase => return true , TwoPhaseActivation :: NotActivated => return false , TwoPhaseActivation :: ActivatedAt (loc) => loc , } ; if activation_location . dominates (location , dominators) { return true ; } let reserve_location = borrow_data . reserve_location . successor_within_block () ; if reserve_location . dominates (location , dominators) { false } else { true } }
    };
}

is_active!()