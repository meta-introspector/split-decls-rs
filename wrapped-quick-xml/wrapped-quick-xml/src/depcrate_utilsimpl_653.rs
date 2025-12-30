// Generated macro for impl_653 (impl)
macro_rules! Depcrate_utilsimpl_653 {
() => {
// Module: crate::utils
// Provides: {"impl_653"}
// Dependencies: {}
# [cfg (feature = "async-tokio")] impl < 'a > tokio :: io :: AsyncBufRead for Fountain < 'a > { # [inline] fn poll_fill_buf (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Poll :: Ready (io :: BufRead :: fill_buf (self . get_mut ())) } # [inline] fn consume (self : Pin < & mut Self > , amt : usize) { io :: BufRead :: consume (self . get_mut () , amt) ; } }
};
}
