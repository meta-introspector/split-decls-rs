// Generated macro for impl_95 (impl)
macro_rules! Depcrate_streamimpl_95 {
() => {
// Module: crate::stream
// Provides: {"impl_95"}
// Dependencies: {}
impl < T , F > Stream for RepeatWith < F > where F : FnMut () -> T , { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let item = (& mut self . f) () ; Poll :: Ready (Some (item)) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
};
}
