// Generated macro for Error (enum)
macro_rules! Depcrate_subscriptionsError {
() => {
// Module: crate::subscriptions
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Possible errors of serving a [`WebSocket`] connection."] # [derive (Debug , Display , StdError)] enum Error { # [doc = " Deserializing of a client [`ws::Message`] failed."] # [display ("`serde` error: {_0}")] Serde (serde_json :: Error) , # [doc = " Unexpected client [`ws::Message`]."] # [display ("unexpected message received from client: {_0:?}")] UnexpectedClientMessage (# [error (not (source))] ws :: Message) , }
};
}
