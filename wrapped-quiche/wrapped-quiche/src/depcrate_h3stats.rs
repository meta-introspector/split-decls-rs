// Generated macro for Stats (struct)
macro_rules! Depcrate_h3Stats {
() => {
// Module: crate::h3
// Provides: {"Stats"}
// Dependencies: {}
# [doc = " Statistics about the connection."] # [doc = ""] # [doc = " A connection's statistics can be collected using the [`stats()`] method."] # [doc = ""] # [doc = " [`stats()`]: struct.Connection.html#method.stats"] # [derive (Clone , Default)] pub struct Stats { # [doc = " The number of bytes received on the QPACK encoder stream."] pub qpack_encoder_stream_recv_bytes : u64 , # [doc = " The number of bytes received on the QPACK decoder stream."] pub qpack_decoder_stream_recv_bytes : u64 , }
};
}
