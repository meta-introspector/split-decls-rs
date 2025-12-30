// Generated macro for UNIX_IO_ERROR_TABLE (const)
macro_rules! Depcrate_shims_io_errorUNIX_IO_ERROR_TABLE {
() => {
// Module: crate::shims::io_error
// Provides: {"UNIX_IO_ERROR_TABLE"}
// Dependencies: {}
const UNIX_IO_ERROR_TABLE : & [(& str , std :: io :: ErrorKind)] = { use std :: io :: ErrorKind :: * ; & [("E2BIG" , ArgumentListTooLong) , ("EADDRINUSE" , AddrInUse) , ("EADDRNOTAVAIL" , AddrNotAvailable) , ("EBUSY" , ResourceBusy) , ("ECONNABORTED" , ConnectionAborted) , ("ECONNREFUSED" , ConnectionRefused) , ("ECONNRESET" , ConnectionReset) , ("EDEADLK" , Deadlock) , ("EDQUOT" , QuotaExceeded) , ("EEXIST" , AlreadyExists) , ("EFBIG" , FileTooLarge) , ("EHOSTUNREACH" , HostUnreachable) , ("EINTR" , Interrupted) , ("EINVAL" , InvalidInput) , ("EISDIR" , IsADirectory) , ("ELOOP" , FilesystemLoop) , ("ENOENT" , NotFound) , ("ENOMEM" , OutOfMemory) , ("ENOSPC" , StorageFull) , ("ENOSYS" , Unsupported) , ("EMLINK" , TooManyLinks) , ("ENAMETOOLONG" , InvalidFilename) , ("ENETDOWN" , NetworkDown) , ("ENETUNREACH" , NetworkUnreachable) , ("ENOTCONN" , NotConnected) , ("ENOTDIR" , NotADirectory) , ("ENOTEMPTY" , DirectoryNotEmpty) , ("EPIPE" , BrokenPipe) , ("EROFS" , ReadOnlyFilesystem) , ("ESPIPE" , NotSeekable) , ("ESTALE" , StaleNetworkFileHandle) , ("ETIMEDOUT" , TimedOut) , ("ETXTBSY" , ExecutableFileBusy) , ("EXDEV" , CrossesDevices) , ("EPERM" , PermissionDenied) , ("EACCES" , PermissionDenied) , ("EWOULDBLOCK" , WouldBlock) , ("EAGAIN" , WouldBlock) ,] } ;
};
}
