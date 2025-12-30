// Generated macro for read (function)
macro_rules! Depcrate_rt_ioread {
() => {
// Module: crate::rt::io
// Provides: {"read"}
// Dependencies: {}
pub (crate) async fn read < T > (io : & mut T , buf : & mut [u8]) -> Result < usize , std :: io :: Error > where T : Read + Unpin , { poll_fn (move | cx | { let mut buf = ReadBuf :: new (buf) ; ready ! (Pin :: new (& mut * io) . poll_read (cx , buf . unfilled ())) ? ; Poll :: Ready (Ok (buf . filled () . len ())) }) . await }
};
}
