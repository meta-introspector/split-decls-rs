// Generated macro for impl_34 (impl)
macro_rules! Depcrate_streamimpl_34 {
() => {
// Module: crate::stream
// Provides: {"impl_34"}
// Dependencies: {}
impl From < Error > for io :: Error { # [inline] fn from (e : Error) -> io :: Error { let kind = match e { Error :: Data => io :: ErrorKind :: InvalidData , Error :: Options => io :: ErrorKind :: InvalidInput , Error :: Format => io :: ErrorKind :: InvalidData , Error :: MemLimit => io :: ErrorKind :: Other , Error :: Mem => io :: ErrorKind :: Other , Error :: Program => io :: ErrorKind :: Other , Error :: NoCheck => io :: ErrorKind :: InvalidInput , Error :: UnsupportedCheck => io :: ErrorKind :: Other , } ; io :: Error :: new (kind , e) } }
};
}
