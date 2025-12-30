// Generated macro for CandidateKind (enum)
macro_rules! Depcrate_method_probeCandidateKind {
() => {
// Module: crate::method::probe
// Provides: {"CandidateKind"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) enum CandidateKind < 'tcx > { InherentImplCandidate { impl_def_id : DefId , receiver_steps : usize } , ObjectCandidate (ty :: PolyTraitRef < 'tcx >) , TraitCandidate (ty :: PolyTraitRef < 'tcx >) , WhereClauseCandidate (ty :: PolyTraitRef < 'tcx >) , }
};
}
