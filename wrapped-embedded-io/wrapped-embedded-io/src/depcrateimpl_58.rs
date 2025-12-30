// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl From < std :: io :: ErrorKind > for ErrorKind { fn from (value : std :: io :: ErrorKind) -> Self { match value { std :: io :: ErrorKind :: NotFound => ErrorKind :: NotFound , std :: io :: ErrorKind :: PermissionDenied => ErrorKind :: PermissionDenied , std :: io :: ErrorKind :: ConnectionRefused => ErrorKind :: ConnectionRefused , std :: io :: ErrorKind :: ConnectionReset => ErrorKind :: ConnectionReset , std :: io :: ErrorKind :: ConnectionAborted => ErrorKind :: ConnectionAborted , std :: io :: ErrorKind :: NotConnected => ErrorKind :: NotConnected , std :: io :: ErrorKind :: AddrInUse => ErrorKind :: AddrInUse , std :: io :: ErrorKind :: AddrNotAvailable => ErrorKind :: AddrNotAvailable , std :: io :: ErrorKind :: BrokenPipe => ErrorKind :: BrokenPipe , std :: io :: ErrorKind :: AlreadyExists => ErrorKind :: AlreadyExists , std :: io :: ErrorKind :: InvalidInput => ErrorKind :: InvalidInput , std :: io :: ErrorKind :: InvalidData => ErrorKind :: InvalidData , std :: io :: ErrorKind :: TimedOut => ErrorKind :: TimedOut , std :: io :: ErrorKind :: Interrupted => ErrorKind :: Interrupted , std :: io :: ErrorKind :: Unsupported => ErrorKind :: Unsupported , std :: io :: ErrorKind :: OutOfMemory => ErrorKind :: OutOfMemory , std :: io :: ErrorKind :: WriteZero => ErrorKind :: WriteZero , _ => ErrorKind :: Other , } } }
};
}
