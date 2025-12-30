// Generated macro for echo (function)
macro_rules! Depcrate_testsecho {
() => {
// Module: crate::tests
// Provides: {"echo"}
// Dependencies: {}
async fn echo ((mut send , mut recv) : (SendStream , RecvStream)) { loop { # [rustfmt :: skip] let mut bufs = [Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () , Bytes :: new () ,] ; match recv . read_chunks (& mut bufs) . await . expect ("read chunks") { Some (n) => { send . write_all_chunks (& mut bufs [.. n]) . await . expect ("write chunks") ; } None => break , } } let _ = send . finish () ; }
};
}
