// Generated macro for impl_168 (impl)
macro_rules! Depcrate_streamimpl_168 {
() => {
// Module: crate::stream
// Provides: {"impl_168"}
// Dependencies: {}
impl < B > Stream for BodyStream < B > where B : Body , { type Item = Result < Frame < B :: Data > , B :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . project () . body . poll_frame (cx) { Poll :: Ready (Some (frame)) => Poll :: Ready (Some (frame)) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
};
}
