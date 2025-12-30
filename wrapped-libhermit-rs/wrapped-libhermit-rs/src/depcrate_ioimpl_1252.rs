// Generated macro for impl_1252 (impl)
macro_rules! Depcrate_ioimpl_1252 {
() => {
// Module: crate::io
// Provides: {"impl_1252"}
// Dependencies: {}
impl From < Errno > for ErrorKind { fn from (value : Errno) -> Self { match value { Errno :: Noent => ErrorKind :: NotFound , Errno :: Acces | Errno :: Perm => ErrorKind :: PermissionDenied , Errno :: Connrefused => ErrorKind :: ConnectionRefused , Errno :: Connreset => ErrorKind :: ConnectionReset , Errno :: Connaborted => ErrorKind :: ConnectionAborted , Errno :: Notconn => ErrorKind :: NotConnected , Errno :: Addrinuse => ErrorKind :: AddrInUse , Errno :: Addrnotavail => ErrorKind :: AddrNotAvailable , Errno :: Pipe => ErrorKind :: BrokenPipe , Errno :: Exist => ErrorKind :: AlreadyExists , Errno :: Inval => ErrorKind :: InvalidInput , Errno :: Timedout => ErrorKind :: TimedOut , Errno :: Intr => ErrorKind :: Interrupted , Errno :: Opnotsupp => ErrorKind :: Unsupported , Errno :: Nomem => ErrorKind :: OutOfMemory , _ => ErrorKind :: Other , } } }
};
}
