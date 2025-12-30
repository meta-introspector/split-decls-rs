// Generated macro for impl_752 (impl)
macro_rules! Depcrate_stream_stream_groupimpl_752 {
() => {
// Module: crate::stream::stream_group
// Provides: {"impl_752"}
// Dependencies: {}
impl < S : Stream > Stream for Keyed < S > { type Item = (Key , < S as Stream > :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; this . group . as_mut () . poll_next_inner (cx) } }
};
}
