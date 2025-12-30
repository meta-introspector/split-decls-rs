// Generated macro for impl_426 (impl)
macro_rules! Depcrate_reader_async_tokioimpl_426 {
() => {
// Module: crate::reader::async_tokio
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'r , R > AsyncBufRead for BinaryStream < 'r , R > where R : AsyncBufRead + Unpin , { # [inline] fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { Pin :: new (& mut * self . get_mut () . inner) . poll_fill_buf (cx) } # [inline] fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . get_mut () ; this . inner . consume (amt) ; * this . offset += amt as u64 ; } }
};
}
