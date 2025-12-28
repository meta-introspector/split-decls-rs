macro_rules! deps {
    () => {
        CandidateKind!();
    };
}

macro_rules! Candidate {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct Candidate < 'tcx > { pub (crate) item : ty :: AssocItem , pub (crate) kind : CandidateKind < 'tcx > , pub (crate) import_ids : SmallVec < [LocalDefId ; 1] > , }
    };
}

Candidate!();