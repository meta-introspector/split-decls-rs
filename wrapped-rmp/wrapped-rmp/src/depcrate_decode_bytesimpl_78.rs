// Generated macro for impl_78 (impl)
macro_rules! Depcrate_decode_bytesimpl_78 {
() => {
// Module: crate::decode::bytes
// Provides: {"impl_78"}
// Dependencies: {}
impl Display for BytesReadError { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { match * self { Self :: InsufficientBytes { expected , actual , position } => { write ! (f , "Expected at least bytes {expected}, but only got {actual} (pos {position})") } } } }
};
}
