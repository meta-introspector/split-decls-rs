// Generated macro for Error (enum)
macro_rules! Depcrate_subscriptionsError {
() => {
// Module: crate::subscriptions
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Errors that can happen while serving a connection."] # [derive (Debug , Display , From , StdError)] pub enum Error { # [doc = " Errors that can happen in Warp while serving a connection."] # [display ("`warp` error: {_0}")] Warp (warp :: Error) , # [doc = " Errors that can happen while serializing outgoing messages. Note that errors that occur"] # [doc = " while deserializing incoming messages are handled internally by the protocol."] # [display ("`serde` error: {_0}")] Serde (serde_json :: Error) , }
};
}
