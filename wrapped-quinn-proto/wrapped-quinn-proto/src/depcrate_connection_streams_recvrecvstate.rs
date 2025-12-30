// Generated macro for RecvState (enum)
macro_rules! Depcrate_connection_streams_recvRecvState {
() => {
// Module: crate::connection::streams::recv
// Provides: {"RecvState"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , PartialEq)] enum RecvState { Recv { size : Option < u64 > } , ResetRecvd { size : u64 , error_code : VarInt } , }
};
}
