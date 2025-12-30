// Generated macro for StreamRecv (enum)
macro_rules! Depcrate_connection_streams_stateStreamRecv {
() => {
// Module: crate::connection::streams::state
// Provides: {"StreamRecv"}
// Dependencies: {}
# [doc = " Wrapper around `Recv` that facilitates reusing `Recv` instances"] # [derive (Debug)] pub (super) enum StreamRecv { # [doc = " A `Recv` that is ready to be opened"] Free (Box < Recv >) , # [doc = " A `Recv` that has been opened"] Open (Box < Recv >) , }
};
}
