// Generated macro for drain_stream (function)
macro_rules! Depcrate_clientdrain_stream {
() => {
// Module: crate::client
// Provides: {"drain_stream"}
// Dependencies: {}
async fn drain_stream (mut stream : quinn :: RecvStream , download : u64 , stream_stats : OpenStreamStats ,) -> Result < () > { if download == 0 { return Ok (()) ; } # [rustfmt :: skip] let mut bufs = [Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () ,] ; let download_start = Instant :: now () ; let recv_stream_stats = stream_stats . new_receiver (& stream , download) ; let mut first_byte = true ; while let Some (size) = stream . read_chunks (& mut bufs [..]) . await ? { if first_byte { recv_stream_stats . on_first_byte (download_start . elapsed ()) ; first_byte = false ; } let bytes_received = bufs [.. size] . iter () . map (| b | b . len ()) . sum () ; recv_stream_stats . on_bytes (bytes_received) ; } if first_byte { recv_stream_stats . on_first_byte (download_start . elapsed ()) ; } recv_stream_stats . finish (download_start . elapsed ()) ; debug ! ("response finished on {}" , stream . id ()) ; Ok (()) }
};
}
