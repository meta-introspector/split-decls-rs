// Generated macro for rustls_io_result (struct)
macro_rules! Depcrate_errorrustls_io_result {
() => {
// Module: crate::error
// Provides: {"rustls_io_result"}
// Dependencies: {}
# [doc = " A return value for a function that may return either success (0) or a"] # [doc = " non-zero value representing an error."] # [doc = ""] # [doc = " The values should match socket error numbers for your operating system --"] # [doc = " for example, the integers for `ETIMEDOUT`, `EAGAIN`, or similar."] # [repr (transparent)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct rustls_io_result (pub libc :: c_int) ;
};
}
