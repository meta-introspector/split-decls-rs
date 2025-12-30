// Generated macro for impl_29 (impl)
macro_rules! Depcrate_eventsource_futuresimpl_29 {
() => {
// Module: crate::eventsource::futures
// Provides: {"impl_29"}
// Dependencies: {}
impl Stream for EventSourceSubscription { type Item = Result < (String , MessageEvent) , EventSourceError > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let msg = ready ! (self . project () . message_receiver . poll_next (cx)) ; match msg { Some (StreamMessage :: Message (event_type , msg)) => { Poll :: Ready (Some (Ok ((event_type , msg)))) } Some (StreamMessage :: ErrorEvent) => { Poll :: Ready (Some (Err (EventSourceError :: ConnectionError))) } None => Poll :: Ready (None) , } } }
};
}
