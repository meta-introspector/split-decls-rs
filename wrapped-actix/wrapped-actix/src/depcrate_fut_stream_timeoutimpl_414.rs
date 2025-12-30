// Generated macro for impl_414 (impl)
macro_rules! Depcrate_fut_stream_timeoutimpl_414 {
() => {
// Module: crate::fut::stream::timeout
// Provides: {"impl_414"}
// Dependencies: {}
impl < S , A > ActorStream < A > for Timeout < S > where S : ActorStream < A > , A : Actor , { type Item = Result < S :: Item , () > ; fn poll_next (self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Option < Result < S :: Item , () > > > { let mut this = self . project () ; match this . stream . poll_next (act , ctx , task) { Poll :: Ready (Some (res)) => { * this . reset_timeout = true ; Poll :: Ready (Some (Ok (res))) } Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => { if * this . reset_timeout { * this . reset_timeout = false ; this . timeout . as_mut () . reset (Instant :: now () + * this . dur) ; } this . timeout . poll (task) . map (| _ | Some (Err (()))) } } } }
};
}
