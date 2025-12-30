// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl From < ErrorKind > for std :: io :: ErrorKind { fn from (value : ErrorKind) -> Self { match value { ErrorKind :: NotFound => std :: io :: ErrorKind :: NotFound , ErrorKind :: PermissionDenied => std :: io :: ErrorKind :: PermissionDenied , ErrorKind :: ConnectionRefused => std :: io :: ErrorKind :: ConnectionRefused , ErrorKind :: ConnectionReset => std :: io :: ErrorKind :: ConnectionReset , ErrorKind :: ConnectionAborted => std :: io :: ErrorKind :: ConnectionAborted , ErrorKind :: NotConnected => std :: io :: ErrorKind :: NotConnected , ErrorKind :: AddrInUse => std :: io :: ErrorKind :: AddrInUse , ErrorKind :: AddrNotAvailable => std :: io :: ErrorKind :: AddrNotAvailable , ErrorKind :: BrokenPipe => std :: io :: ErrorKind :: BrokenPipe , ErrorKind :: AlreadyExists => std :: io :: ErrorKind :: AlreadyExists , ErrorKind :: InvalidInput => std :: io :: ErrorKind :: InvalidInput , ErrorKind :: InvalidData => std :: io :: ErrorKind :: InvalidData , ErrorKind :: TimedOut => std :: io :: ErrorKind :: TimedOut , ErrorKind :: Interrupted => std :: io :: ErrorKind :: Interrupted , ErrorKind :: Unsupported => std :: io :: ErrorKind :: Unsupported , ErrorKind :: OutOfMemory => std :: io :: ErrorKind :: OutOfMemory , ErrorKind :: WriteZero => std :: io :: ErrorKind :: WriteZero , _ => std :: io :: ErrorKind :: Other , } } }
};
}
