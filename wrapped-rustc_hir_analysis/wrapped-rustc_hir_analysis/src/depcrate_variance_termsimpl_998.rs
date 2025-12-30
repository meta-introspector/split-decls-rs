// Generated macro for impl_998 (impl)
macro_rules! Depcrate_variance_termsimpl_998 {
() => {
// Module: crate::variance::terms
// Provides: {"impl_998"}
// Dependencies: {}
impl < 'a , 'tcx > TermsContext < 'a , 'tcx > { fn add_inferreds_for_item (& mut self , def_id : LocalDefId) { let tcx = self . tcx ; let count = tcx . generics_of (def_id) . count () ; if count == 0 { return ; } let start = self . inferred_terms . len () ; let newly_added = self . inferred_starts . insert (def_id , InferredIndex (start)) . is_none () ; assert ! (newly_added) ; let arena = self . arena ; self . inferred_terms . extend ((start .. (start + count)) . map (| i | & * arena . alloc (InferredTerm (InferredIndex (i)))) ,) ; } }
};
}
