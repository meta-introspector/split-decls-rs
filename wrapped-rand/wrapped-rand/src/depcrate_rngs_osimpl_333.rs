// Generated macro for impl_333 (impl)
macro_rules! Depcrate_rngs_osimpl_333 {
() => {
// Module: crate::rngs::os
// Provides: {"impl_333"}
// Dependencies: {}
impl OsError { # [doc = " Extract the raw OS error code (if this error came from the OS)"] # [doc = ""] # [doc = " This method is identical to [`std::io::Error::raw_os_error()`][1], except"] # [doc = " that it works in `no_std` contexts. If this method returns `None`, the"] # [doc = " error value can still be formatted via the `Display` implementation."] # [doc = ""] # [doc = " [1]: https://doc.rust-lang.org/std/io/struct.Error.html#method.raw_os_error"] # [inline] pub fn raw_os_error (self) -> Option < RawOsError > { self . 0 . raw_os_error () } }
};
}
