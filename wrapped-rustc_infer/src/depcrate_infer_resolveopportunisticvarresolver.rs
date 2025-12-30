// Generated macro for OpportunisticVarResolver (struct)
macro_rules! Depcrate_infer_resolveOpportunisticVarResolver {
() => {
// Module: crate::infer::resolve
// Provides: {"OpportunisticVarResolver"}
// Dependencies: {}
# [doc = " The opportunistic resolver can be used at any time. It simply replaces"] # [doc = " type/const variables that have been unified with the things they have"] # [doc = " been unified with (similar to `shallow_resolve`, but deep). This is"] # [doc = " useful for printing messages etc but also required at various"] # [doc = " points for correctness."] pub struct OpportunisticVarResolver < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , # [doc = " We're able to use a cache here as the folder does"] # [doc = " not have any mutable state."] cache : DelayedMap < Ty < 'tcx > , Ty < 'tcx > > , }
};
}
