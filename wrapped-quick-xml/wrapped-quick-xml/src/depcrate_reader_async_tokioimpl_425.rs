// Generated macro for impl_425 (impl)
macro_rules! Depcrate_reader_async_tokioimpl_425 {
() => {
// Module: crate::reader::async_tokio
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'r , R > AsyncRead for BinaryStream < 'r , R > where R : AsyncRead + Unpin , { fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let start = buf . remaining () ; let this = self . get_mut () ; let poll = Pin :: new (& mut * this . inner) . poll_read (cx , buf) ; if let Poll :: Ready (Ok (_)) = poll { let amt = start - buf . remaining () ; * this . offset += amt as u64 ; } poll } }
};
}
