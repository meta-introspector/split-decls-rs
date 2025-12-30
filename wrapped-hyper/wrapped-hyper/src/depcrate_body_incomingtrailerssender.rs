// Generated macro for TrailersSender (type)
macro_rules! Depcrate_body_incomingTrailersSender {
() => {
// Module: crate::body::incoming
// Provides: {"TrailersSender"}
// Dependencies: {}
# [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] type TrailersSender = oneshot :: Sender < HeaderMap > ;
};
}
