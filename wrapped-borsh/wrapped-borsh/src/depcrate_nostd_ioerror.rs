// Generated macro for Error (struct)
macro_rules! Depcrate_nostd_ioError {
() => {
// Module: crate::nostd_io
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error type for I/O operations of the [`Read`], [`Write`], [`Seek`], and"] # [doc = " associated traits."] # [doc = ""] # [doc = " Errors mostly originate from the underlying OS, but custom instances of"] # [doc = " `Error` can be created with crafted error messages and a particular value of"] # [doc = " [`ErrorKind`]."] # [doc = ""] # [doc = " [`Read`]: crate::io::Read"] # [doc = " [`Write`]: crate::io::Write"] # [doc = " [`Seek`]: crate::io::Seek"] pub struct Error { repr : Repr , }
};
}
