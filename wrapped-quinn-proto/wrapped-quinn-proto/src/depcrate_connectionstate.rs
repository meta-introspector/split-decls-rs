// Generated macro for State (enum)
macro_rules! Depcrate_connectionState {
() => {
// Module: crate::connection
// Provides: {"State"}
// Dependencies: {}
# [allow (unreachable_pub)] # [derive (Clone)] pub enum State { Handshake (state :: Handshake) , Established , Closed (state :: Closed) , Draining , # [doc = " Waiting for application to call close so we can dispose of the resources"] Drained , }
};
}
