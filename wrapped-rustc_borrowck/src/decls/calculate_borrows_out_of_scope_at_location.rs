macro_rules! deps {
    () => {
        RegionInferenceContext!();
        BorrowSet!();
        OutOfScopePrecomputer!();
    };
}

macro_rules! calculate_borrows_out_of_scope_at_location {
    () => {
        deps!();
        pub fn calculate_borrows_out_of_scope_at_location < 'tcx > (body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , borrow_set : & BorrowSet < 'tcx > ,) -> FxIndexMap < Location , Vec < BorrowIndex > > { OutOfScopePrecomputer :: compute (body , regioncx , borrow_set) }
    };
}

calculate_borrows_out_of_scope_at_location!()