// Generated macro for EventSourceSubscription (struct)
macro_rules! Depcrate_eventsource_futuresEventSourceSubscription {
() => {
// Module: crate::eventsource::futures
// Provides: {"EventSourceSubscription"}
// Dependencies: {}
# [doc = " Wrapper around browser's EventSource API."] # [pin_project (PinnedDrop)] pub struct EventSourceSubscription { # [allow (clippy :: type_complexity)] error_callback : Closure < dyn FnMut (web_sys :: Event) > , es : web_sys :: EventSource , event_type : String , message_callback : Closure < dyn FnMut (MessageEvent) > , # [pin] message_receiver : mpsc :: UnboundedReceiver < StreamMessage > , }
};
}
