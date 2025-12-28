macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! OutOfScopePrecomputer {
    () => {
        deps!();
        struct OutOfScopePrecomputer < 'a , 'tcx > { visited : DenseBitSet < mir :: BasicBlock > , visit_stack : Vec < mir :: BasicBlock > , body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , borrows_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }
    };
}

OutOfScopePrecomputer!()