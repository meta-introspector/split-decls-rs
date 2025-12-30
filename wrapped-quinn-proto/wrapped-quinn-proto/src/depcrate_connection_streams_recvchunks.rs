// Generated macro for Chunks (struct)
macro_rules! Depcrate_connection_streams_recvChunks {
() => {
// Module: crate::connection::streams::recv
// Provides: {"Chunks"}
// Dependencies: {}
# [doc = " Chunks returned from [`RecvStream::read()`][crate::RecvStream::read]."] # [doc = ""] # [doc = " ### Note: Finalization Needed"] # [doc = " Bytes read from the stream are not released from the congestion window until"] # [doc = " either [`Self::finalize()`] is called, or this type is dropped."] # [doc = ""] # [doc = " It is recommended that you call [`Self::finalize()`] because it returns a flag"] # [doc = " telling you whether reading from the stream has resulted in the need to transmit a packet."] # [doc = ""] # [doc = " If this type is leaked, the stream will remain blocked on the remote peer until"] # [doc = " another read from the stream is done."] pub struct Chunks < 'a > { id : StreamId , ordered : bool , streams : & 'a mut StreamsState , pending : & 'a mut Retransmits , state : ChunksState , read : u64 , }
};
}
