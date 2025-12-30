// Generated macro for SendState (enum)
macro_rules! Depcrate_connection_streams_sendSendState {
() => {
// Module: crate::connection::streams::send
// Provides: {"SendState"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (super) enum SendState { # [doc = " Sending new data"] Ready , # [doc = " Stream was finished; now sending retransmits only"] DataSent { finish_acked : bool } , # [doc = " Sent RESET"] ResetSent , }
};
}
