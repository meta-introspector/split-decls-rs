// Generated macro for impl_861 (impl)
macro_rules! Depcrate_stream_merge_vecimpl_861 {
() => {
// Module: crate::stream::merge::vec
// Provides: {"impl_861"}
// Dependencies: {}
impl < S > MergeTrait for Vec < S > where S : IntoStream , { type Item = < Merge < S :: IntoStream > as Stream > :: Item ; type Stream = Merge < S :: IntoStream > ; fn merge (self) -> Self :: Stream { Merge :: new (self . into_iter () . map (| i | i . into_stream ()) . collect ()) } }
};
}
