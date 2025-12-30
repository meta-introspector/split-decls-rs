// Generated macro for ConsumerOptions (enum)
macro_rules! Depcrate_consumersConsumerOptions {
() => {
// Module: crate::consumers
// Provides: {"ConsumerOptions"}
// Dependencies: {}
# [doc = " Options determining the output behavior of [`get_bodies_with_borrowck_facts`]."] # [doc = ""] # [doc = " If executing under `-Z polonius` the choice here has no effect, and everything as if"] # [doc = " [`PoloniusOutputFacts`](ConsumerOptions::PoloniusOutputFacts) had been selected"] # [doc = " will be retrieved."] # [derive (Debug , Copy , Clone)] pub enum ConsumerOptions { # [doc = " Retrieve the [`Body`] along with the [`BorrowSet`]"] # [doc = " and [`RegionInferenceContext`]. If you would like the body only, use"] # [doc = " [`TyCtxt::mir_promoted`]."] # [doc = ""] # [doc = " These can be used in conjunction with [`calculate_borrows_out_of_scope_at_location`]."] RegionInferenceContext , # [doc = " The recommended option. Retrieves the maximal amount of information"] # [doc = " without significant slowdowns."] # [doc = ""] # [doc = " Implies [`RegionInferenceContext`](ConsumerOptions::RegionInferenceContext),"] # [doc = " and additionally retrieve the [`PoloniusLocationTable`] and [`PoloniusInput`] that"] # [doc = " would be given to Polonius. Critically, this does not run Polonius, which"] # [doc = " one may want to avoid due to performance issues on large bodies."] PoloniusInputFacts , # [doc = " Implies [`PoloniusInputFacts`](ConsumerOptions::PoloniusInputFacts),"] # [doc = " and additionally runs Polonius to calculate the [`PoloniusOutput`]."] PoloniusOutputFacts , }
};
}
