macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! PoloniusOutOfScopePrecomputer {
    () => {
        deps!();
        struct PoloniusOutOfScopePrecomputer < 'a , 'tcx > { visited : DenseBitSet < mir :: BasicBlock > , visit_stack : Vec < mir :: BasicBlock > , body : & 'a Body < 'tcx > , regioncx : & 'a RegionInferenceContext < 'tcx > , loans_out_of_scope_at_location : FxIndexMap < Location , Vec < BorrowIndex > > , }
    };
}

PoloniusOutOfScopePrecomputer!();