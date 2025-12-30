// Generated macro for impl_20 (impl)
macro_rules! Depcrate_nextimpl_20 {
() => {
// Module: crate::next
// Provides: {"impl_20"}
// Dependencies: {}
impl < S > Future for Next < '_ , S > where S : Stream + Unpin , { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . stream) . poll_next (cx) } }
};
}
