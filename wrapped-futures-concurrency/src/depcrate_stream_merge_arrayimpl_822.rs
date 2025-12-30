// Generated macro for impl_822 (impl)
macro_rules! Depcrate_stream_merge_arrayimpl_822 {
() => {
// Module: crate::stream::merge::array
// Provides: {"impl_822"}
// Dependencies: {}
impl < S , const N : usize > MergeTrait for [S ; N] where S : IntoStream , { type Item = < Merge < S :: IntoStream , N > as Stream > :: Item ; type Stream = Merge < S :: IntoStream , N > ; fn merge (self) -> Self :: Stream { Merge :: new (self . map (| i | i . into_stream ())) } }
};
}
