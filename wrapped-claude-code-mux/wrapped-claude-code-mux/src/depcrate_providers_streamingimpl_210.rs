// Generated macro for impl_210 (impl)
macro_rules! Depcrate_providers_streamingimpl_210 {
() => {
// Module: crate::providers::streaming
// Provides: {"impl_210"}
// Dependencies: {}
impl < S > Stream for SseStream < S > where S : Stream < Item = Result < Bytes , reqwest :: Error > > , { type Item = Result < SseEvent , reqwest :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match this . inner . poll_next (cx) { Poll :: Ready (Some (Ok (bytes))) => { if let Ok (text) = std :: str :: from_utf8 (& bytes) { this . buffer . push_str (text) ; let events = parse_sse_events (this . buffer) ; if let Some (event) = events . first () { * this . buffer = String :: new () ; return Poll :: Ready (Some (Ok (event . clone ()))) ; } } cx . waker () . wake_by_ref () ; Poll :: Pending } Poll :: Ready (Some (Err (e))) => Poll :: Ready (Some (Err (e))) , Poll :: Ready (None) => { if ! this . buffer . is_empty () { let events = parse_sse_events (this . buffer) ; * this . buffer = String :: new () ; if let Some (event) = events . first () { return Poll :: Ready (Some (Ok (event . clone ()))) ; } } Poll :: Ready (None) } Poll :: Pending => Poll :: Pending , } } }
};
}
