// Generated macro for impl_30 (impl)
macro_rules! Depcrate_core_errorimpl_30 {
() => {
// Module: crate::core::error
// Provides: {"impl_30"}
// Dependencies: {}
impl From < Error > for io :: Error { # [doc = " Convert the [`Error`] to an [`io::Error`], preserving the original"] # [doc = " [`Error`] as the [\"inner error\"]. Note that this also makes the display"] # [doc = " of the error include the context."] # [doc = ""] # [doc = " This is different from [`into_io_error`] which returns the original"] # [doc = " [`io::Error`]."] # [doc = ""] # [doc = " [`Error`]: struct.Error.html"] # [doc = " [`io::Error`]: https://doc.rust-lang.org/stable/std/io/struct.Error.html"] # [doc = " [\"inner error\"]: https://doc.rust-lang.org/std/io/struct.Error.html#method.into_inner"] # [doc = " [`into_io_error`]: struct.WalkDir.html#method.into_io_error"] fn from (walk_err : Error) -> io :: Error { let kind = match walk_err { Error { inner : ErrorInner :: Io { ref err , .. } , .. } => err . kind () , Error { inner : ErrorInner :: Loop { .. } , .. } => io :: ErrorKind :: Other , Error { inner : ErrorInner :: ThreadpoolBusy , .. } => io :: ErrorKind :: Other , } ; io :: Error :: new (kind , walk_err) } }
};
}
