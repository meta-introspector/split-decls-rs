macro_rules! CandidateKind {
    () => {
        # [derive (Debug , Clone)] pub (crate) enum CandidateKind < 'tcx > { InherentImplCandidate { impl_def_id : DefId , receiver_steps : usize } , ObjectCandidate (ty :: PolyTraitRef < 'tcx >) , TraitCandidate (ty :: PolyTraitRef < 'tcx >) , WhereClauseCandidate (ty :: PolyTraitRef < 'tcx >) , }
    };
}

CandidateKind!();