// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (feature = "std")] impl AsyncBufRead for Reader { fn poll_fill_buf (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { ready ! (self . poll (cx)) ; let this = unsafe { self . get_unchecked_mut () } ; Poll :: Ready (Ok (this . peek_buf ())) } fn consume (mut self : Pin < & mut Self > , amt : usize) { (* self) . consume (amt) } }
};
}
