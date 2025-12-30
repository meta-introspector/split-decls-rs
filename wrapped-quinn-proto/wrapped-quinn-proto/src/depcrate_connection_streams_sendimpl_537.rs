// Generated macro for impl_537 (impl)
macro_rules! Depcrate_connection_streams_sendimpl_537 {
() => {
// Module: crate::connection::streams::send
// Provides: {"impl_537"}
// Dependencies: {}
impl BytesSource for BytesArray < '_ > { fn pop_chunk (& mut self , limit : usize) -> (Bytes , usize) { let mut chunks_consumed = 0 ; while self . consumed < self . chunks . len () { let chunk = & mut self . chunks [self . consumed] ; if chunk . len () <= limit { let chunk = std :: mem :: take (chunk) ; self . consumed += 1 ; chunks_consumed += 1 ; if chunk . is_empty () { continue ; } return (chunk , chunks_consumed) ; } else if limit > 0 { let chunk = chunk . split_to (limit) ; return (chunk , chunks_consumed) ; } else { break ; } } (Bytes :: new () , chunks_consumed) } }
};
}
