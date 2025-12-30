// Generated macro for FinderBuilder (struct)
macro_rules! Depcrate_memmemFinderBuilder {
() => {
// Module: crate::memmem
// Provides: {"FinderBuilder"}
// Dependencies: {}
# [doc = " A builder for constructing non-default forward or reverse memmem finders."] # [doc = ""] # [doc = " A builder is primarily useful for configuring a substring searcher."] # [doc = " Currently, the only configuration exposed is the ability to disable"] # [doc = " heuristic prefilters used to speed up certain searches."] # [derive (Clone , Debug , Default)] pub struct FinderBuilder { prefilter : Prefilter , }
};
}
