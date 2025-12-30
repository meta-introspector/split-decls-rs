// Generated macro for impl_1550 (impl)
macro_rules! Depcrate_stream_repeat_withimpl_1550 {
() => {
// Module: crate::stream::repeat_with
// Provides: {"impl_1550"}
// Dependencies: {}
impl < A , F : FnMut () -> A > Stream for RepeatWith < F > { type Item = A ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some ((& mut self . repeater) ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
