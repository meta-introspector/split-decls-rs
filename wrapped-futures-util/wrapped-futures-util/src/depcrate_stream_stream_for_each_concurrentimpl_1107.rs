// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_stream_stream_for_each_concurrentimpl_1107 {
() => {
// Module: crate::stream::stream::for_each_concurrent
// Provides: {"impl_1107"}
// Dependencies: {}
impl < St , Fut , F > Future for ForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let mut this = self . project () ; loop { let mut made_progress_this_iter = false ; if this . limit . map (| limit | limit . get () > this . futures . len ()) . unwrap_or (true) { let mut stream_completed = false ; let elem = if let Some (stream) = this . stream . as_mut () . as_pin_mut () { match stream . poll_next (cx) { Poll :: Ready (Some (elem)) => { made_progress_this_iter = true ; Some (elem) } Poll :: Ready (None) => { stream_completed = true ; None } Poll :: Pending => None , } } else { None } ; if stream_completed { this . stream . set (None) ; } if let Some (elem) = elem { this . futures . push ((this . f) (elem)) ; } } match this . futures . poll_next_unpin (cx) { Poll :: Ready (Some (())) => made_progress_this_iter = true , Poll :: Ready (None) => { if this . stream . is_none () { return Poll :: Ready (()) ; } } Poll :: Pending => { } } if ! made_progress_this_iter { return Poll :: Pending ; } } } }
};
}
