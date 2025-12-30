// Generated macro for Event (enum)
macro_rules! Depcrate_httpEvent {
() => {
// Module: crate::http
// Provides: {"Event"}
// Dependencies: {}
# [derive (Debug)] pub enum Event { HttpTransactionSendRequestHeaders (HttpTransactionSendRequestHeadersEvent) , HttpTransactionHttp2SendRequestHeaders (HttpTransactionHttp2SendRequestHeadersEvent ,) , HttpTransactionQuicSendRequestHeaders (HttpTransactionQuicSendRequestHeadersEvent ,) , HttpTransactionReadResponseHeaders (HttpTransactionReadResponseHeadersEvent) , HttpStreamJobBoundToRequest (HttpStreamJobBoundToRequestEvent) , HttpStreamRequestBoundToJob (HttpStreamRequestBoundToJobEvent) , HttpStreamRequestBoundToQuicSession (HttpStreamRequestBoundToQuicSessionEvent) , }
};
}
