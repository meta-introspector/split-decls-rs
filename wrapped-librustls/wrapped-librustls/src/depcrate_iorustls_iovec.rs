// Generated macro for rustls_iovec (struct)
macro_rules! Depcrate_iorustls_iovec {
() => {
// Module: crate::io
// Provides: {"rustls_iovec"}
// Dependencies: {}
# [doc = " An alias for `struct iovec` from uio.h (on Unix) or `WSABUF` on Windows."] # [doc = ""] # [doc = " You should cast `const struct rustls_iovec *` to `const struct iovec *` on"] # [doc = " Unix, or `const *LPWSABUF` on Windows. See [`std::io::IoSlice`] for details"] # [doc = " on interoperability with platform specific vectored IO."] pub struct rustls_iovec { _private : [u8 ; 0] , }
};
}
