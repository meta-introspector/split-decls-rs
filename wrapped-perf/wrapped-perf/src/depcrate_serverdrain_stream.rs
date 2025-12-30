// Generated macro for drain_stream (function)
macro_rules! Depcrate_serverdrain_stream {
() => {
// Module: crate::server
// Provides: {"drain_stream"}
// Dependencies: {}
async fn drain_stream (mut stream : quinn :: RecvStream) -> Result < () > { # [rustfmt :: skip] let mut bufs = [Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () ,] ; while stream . read_chunks (& mut bufs [..]) . await ? . is_some () { } debug ! ("finished reading {}" , stream . id ()) ; Ok (()) }
};
}
