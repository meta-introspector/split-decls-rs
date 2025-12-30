// Generated macro for RttEstimator (struct)
macro_rules! Depcrate_connection_pathsRttEstimator {
() => {
// Module: crate::connection::paths
// Provides: {"RttEstimator"}
// Dependencies: {}
# [doc = " RTT estimation for a particular network path"] # [derive (Copy , Clone)] pub struct RttEstimator { # [doc = " The most recent RTT measurement made when receiving an ack for a previously unacked packet"] latest : Duration , # [doc = " The smoothed RTT of the connection, computed as described in RFC6298"] smoothed : Option < Duration > , # [doc = " The RTT variance, computed as described in RFC6298"] var : Duration , # [doc = " The minimum RTT seen in the connection, ignoring ack delay."] min : Duration , }
};
}
