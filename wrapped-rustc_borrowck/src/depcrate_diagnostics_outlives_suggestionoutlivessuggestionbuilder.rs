// Generated macro for OutlivesSuggestionBuilder (struct)
macro_rules! Depcrate_diagnostics_outlives_suggestionOutlivesSuggestionBuilder {
() => {
// Module: crate::diagnostics::outlives_suggestion
// Provides: {"OutlivesSuggestionBuilder"}
// Dependencies: {}
# [doc = " Collects information about outlives constraints that needed to be added for a given MIR node"] # [doc = " corresponding to a function definition."] # [doc = ""] # [doc = " Adds a help note suggesting adding a where clause with the needed constraints."] # [derive (Default)] pub (crate) struct OutlivesSuggestionBuilder { # [doc = " The list of outlives constraints that need to be added. Specifically, we map each free"] # [doc = " region to all other regions that it must outlive. I will use the shorthand `fr:"] # [doc = " outlived_frs`. Not all of these regions will already have names necessarily. Some could be"] # [doc = " implicit free regions that we inferred. These will need to be given names in the final"] # [doc = " suggestion message."] constraints_to_add : BTreeMap < RegionVid , Vec < RegionVid > > , }
};
}
