// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl FromOsStrError { # [doc = " Converts self into a [`std::io::Error`] with kind"] # [doc = " [`InvalidData`](io::ErrorKind::InvalidData)."] # [doc = ""] # [doc = " Many users of [`FromOsStrError`] will want to convert it into an [`io::Error`]. This is a"] # [doc = " convenience method to do that."] pub fn into_io_error (self) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , self) } }
};
}
