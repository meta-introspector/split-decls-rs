// Generated macro for impl_73 (impl)
macro_rules! Depcrate_search_outcomeimpl_73 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_73"}
// Dependencies: {}
impl MatchLocation { fn to_outer < 'a > (& self , out : & 'a Outcome) -> crate :: search :: MatchLocation < 'a > { crate :: search :: MatchLocation { source : self . source . and_then (| source | out . source_paths . resolve (source) . map (AsRef :: as_ref)) , sequence_number : self . sequence_number , } } }
};
}
