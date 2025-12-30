// Generated macro for impl_1021 (impl)
macro_rules! Depcrate_ssl_errorimpl_1021 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1021"}
// Dependencies: {}
impl ErrorCode { # [doc = " The SSL session has been closed."] pub const ZERO_RETURN : ErrorCode = ErrorCode (ffi :: SSL_ERROR_ZERO_RETURN) ; # [doc = " An attempt to read data from the underlying socket returned `WouldBlock`."] # [doc = ""] # [doc = " Wait for read readiness and retry the operation."] pub const WANT_READ : ErrorCode = ErrorCode (ffi :: SSL_ERROR_WANT_READ) ; # [doc = " An attempt to write data to the underlying socket returned `WouldBlock`."] # [doc = ""] # [doc = " Wait for write readiness and retry the operation."] pub const WANT_WRITE : ErrorCode = ErrorCode (ffi :: SSL_ERROR_WANT_WRITE) ; # [doc = " A non-recoverable IO error occurred."] pub const SYSCALL : ErrorCode = ErrorCode (ffi :: SSL_ERROR_SYSCALL) ; # [doc = " An error occurred in the SSL library."] pub const SSL : ErrorCode = ErrorCode (ffi :: SSL_ERROR_SSL) ; # [doc = " The client hello callback indicated that it needed to be retried."] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [cfg (ossl111)] pub const WANT_CLIENT_HELLO_CB : ErrorCode = ErrorCode (ffi :: SSL_ERROR_WANT_CLIENT_HELLO_CB) ; pub fn from_raw (raw : c_int) -> ErrorCode { ErrorCode (raw) } # [allow (clippy :: trivially_copy_pass_by_ref)] pub fn as_raw (& self) -> c_int { self . 0 } }
};
}
