macro_rules! deps {
    () => {
        ClosureRegionRequirements!();
    };
}

macro_rules! PropagatedBorrowCheckResults {
    () => {
        deps!();
        # [doc = " Data propagated to the typeck parent by nested items."] # [doc = " This should always be empty for the typeck root."] # [derive (Debug)] struct PropagatedBorrowCheckResults < 'tcx > { closure_requirements : Option < ClosureRegionRequirements < 'tcx > > , used_mut_upvars : SmallVec < [FieldIdx ; 8] > , }
    };
}

PropagatedBorrowCheckResults!()