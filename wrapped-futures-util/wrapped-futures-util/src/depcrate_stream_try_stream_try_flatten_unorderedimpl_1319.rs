// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1319 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1319"}
// Dependencies: {}
impl < T > Stream for Single < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . 0 . take ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . as_ref () . map_or ((0 , Some (0)) , | _ | (1 , Some (1))) } }
};
}
