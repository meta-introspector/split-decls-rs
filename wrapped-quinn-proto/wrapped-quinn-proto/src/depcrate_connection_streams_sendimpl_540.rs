// Generated macro for impl_540 (impl)
macro_rules! Depcrate_connection_streams_sendimpl_540 {
() => {
// Module: crate::connection::streams::send
// Provides: {"impl_540"}
// Dependencies: {}
impl BytesSource for ByteSlice < '_ > { fn pop_chunk (& mut self , limit : usize) -> (Bytes , usize) { let limit = limit . min (self . data . len ()) ; if limit == 0 { return (Bytes :: new () , 0) ; } let chunk = Bytes :: from (self . data [.. limit] . to_owned ()) ; self . data = & self . data [chunk . len () ..] ; let chunks_consumed = usize :: from (self . data . is_empty ()) ; (chunk , chunks_consumed) } }
};
}
