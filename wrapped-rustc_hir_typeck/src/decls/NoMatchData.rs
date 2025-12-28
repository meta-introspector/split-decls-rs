macro_rules! deps {
    () => {
        Mode!();
        CandidateSource!();
    };
}

macro_rules! NoMatchData {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct NoMatchData < 'tcx > { pub static_candidates : Vec < CandidateSource > , pub unsatisfied_predicates : Vec < (ty :: Predicate < 'tcx > , Option < ty :: Predicate < 'tcx > > , Option < ObligationCause < 'tcx > >) > , pub out_of_scope_traits : Vec < DefId > , pub similar_candidate : Option < ty :: AssocItem > , pub mode : probe :: Mode , }
    };
}

NoMatchData!();