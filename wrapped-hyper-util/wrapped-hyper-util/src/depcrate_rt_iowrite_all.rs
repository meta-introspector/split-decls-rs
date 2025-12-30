// Generated macro for write_all (function)
macro_rules! Depcrate_rt_iowrite_all {
() => {
// Module: crate::rt::io
// Provides: {"write_all"}
// Dependencies: {}
pub (crate) async fn write_all < T > (io : & mut T , buf : & [u8]) -> Result < () , std :: io :: Error > where T : Write + Unpin , { let mut n = 0 ; poll_fn (move | cx | { while n < buf . len () { n += ready ! (Pin :: new (& mut * io) . poll_write (cx , & buf [n ..]) ?) ; } Poll :: Ready (Ok (())) }) . await }
};
}
