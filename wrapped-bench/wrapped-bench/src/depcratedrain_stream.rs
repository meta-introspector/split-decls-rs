// Generated macro for drain_stream (function)
macro_rules! Depcratedrain_stream {
() => {
// Module: crate
// Provides: {"drain_stream"}
// Dependencies: {}
pub async fn drain_stream (stream : & mut quinn :: RecvStream , read_unordered : bool) -> Result < usize > { let mut read = 0 ; if read_unordered { while let Some (chunk) = stream . read_chunk (usize :: MAX , false) . await ? { read += chunk . bytes . len () ; } } else { # [rustfmt :: skip] let mut bufs = [Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () ,] ; while let Some (n) = stream . read_chunks (& mut bufs [..]) . await ? { read += bufs . iter () . take (n) . map (| buf | buf . len ()) . sum :: < usize > () ; } } Ok (read) }
};
}
