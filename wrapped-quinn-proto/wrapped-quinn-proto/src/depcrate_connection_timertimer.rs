// Generated macro for Timer (enum)
macro_rules! Depcrate_connection_timerTimer {
() => {
// Module: crate::connection::timer
// Provides: {"Timer"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Ord , PartialOrd , Eq , PartialEq)] pub (crate) enum Timer { # [doc = " When to send an ack-eliciting probe packet or declare unacked packets lost"] LossDetection = 0 , # [doc = " When to close the connection after no activity"] Idle = 1 , # [doc = " When the close timer expires, the connection has been gracefully terminated."] Close = 2 , # [doc = " When keys are discarded because they should not be needed anymore"] KeyDiscard = 3 , # [doc = " When to give up on validating a new path to the peer"] PathValidation = 4 , # [doc = " When to send a `PING` frame to keep the connection alive"] KeepAlive = 5 , # [doc = " When pacing will allow us to send a packet"] Pacing = 6 , # [doc = " When to invalidate old CID and proactively push new one via NEW_CONNECTION_ID frame"] PushNewCid = 7 , # [doc = " When to send an immediate ACK if there are unacked ack-eliciting packets of the peer"] MaxAckDelay = 8 , }
};
}
