// Generated macro for ChunksState (enum)
macro_rules! Depcrate_connection_streams_recvChunksState {
() => {
// Module: crate::connection::streams::recv
// Provides: {"ChunksState"}
// Dependencies: {}
enum ChunksState { Readable (Box < Recv >) , Reset (VarInt) , Finished , Finalized , }
};
}
