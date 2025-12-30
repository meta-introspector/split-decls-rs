// Generated macro for tokio_03_to_read_buf (function)
macro_rules! Depcrate_stream_buf_readertokio_03_to_read_buf {
() => {
// Module: crate::stream::buf_reader
// Provides: {"tokio_03_to_read_buf"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] fn tokio_03_to_read_buf (bs : & mut BytesMut) -> tokio_03_dep :: io :: ReadBuf < '_ > { let uninit = bs . chunk_mut () ; unsafe { tokio_03_dep :: io :: ReadBuf :: uninit (std :: slice :: from_raw_parts_mut (uninit . as_mut_ptr () as * mut MaybeUninit < u8 > , uninit . len () ,)) } }
};
}
