// Generated macro for impl_383 (impl)
macro_rules! Depcrate_fut_stream_mapimpl_383 {
() => {
// Module: crate::fut::stream::map
// Provides: {"impl_383"}
// Dependencies: {}
impl < S , A , F , U > ActorStream < A > for Map < S , F > where S : ActorStream < A > , A : Actor , F : FnMut (S :: Item , & mut A , & mut A :: Context) -> U , { type Item = U ; fn poll_next (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let res = ready ! (this . stream . as_mut () . poll_next (act , ctx , task)) ; Poll :: Ready (res . map (| x | (this . f) (x , act , ctx))) } }
};
}
