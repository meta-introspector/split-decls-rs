// Generated macro for BodySender (type)
macro_rules! Depcrate_body_incomingBodySender {
() => {
// Module: crate::body::incoming
// Provides: {"BodySender"}
// Dependencies: {}
# [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] type BodySender = mpsc :: Sender < Result < Bytes , crate :: Error > > ;
};
}
