// Generated macro for OpportunisticRegionResolver (struct)
macro_rules! Depcrate_infer_resolveOpportunisticRegionResolver {
() => {
// Module: crate::infer::resolve
// Provides: {"OpportunisticRegionResolver"}
// Dependencies: {}
# [doc = " The opportunistic region resolver opportunistically resolves regions"] # [doc = " variables to the variable with the least variable id. It is used when"] # [doc = " normalizing projections to avoid hitting the recursion limit by creating"] # [doc = " many versions of a predicate for types that in the end have to unify."] # [doc = ""] # [doc = " If you want to resolve type and const variables as well, call"] # [doc = " [InferCtxt::resolve_vars_if_possible] first."] pub struct OpportunisticRegionResolver < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , }
};
}
