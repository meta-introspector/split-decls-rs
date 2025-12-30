// Generated macro for ProgressId (enum)
macro_rules! Depcrate_index_traverse_typesProgressId {
() => {
// Module: crate::index::traverse::types
// Provides: {"ProgressId"}
// Dependencies: {}
# [doc = " The progress ids used in [`traverse()`][crate::index::File::traverse()] ."] # [doc = ""] # [doc = " Use this information to selectively extract the progress of interest in case the parent application has custom visualization."] # [derive (Debug , Copy , Clone)] pub enum ProgressId { # [doc = " A root progress which isn't actually used, but links to the `ProgressId` of the lookup version of the algorithm."] WithLookup (PhantomData < super :: with_lookup :: ProgressId >) , # [doc = " A root progress which isn't actually used, but links to the `ProgressId` of the indexed version of the algorithm."] WithIndex (PhantomData < super :: with_index :: ProgressId >) , }
};
}
