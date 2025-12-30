// Generated macro for impl_121 (impl)
macro_rules! Depcrateimpl_121 {
() => {
// Module: crate
// Provides: {"impl_121"}
// Dependencies: {}
impl FromOsStringError { # [doc = " Returns the [`OsStr`] slice that was attempted to be converted to [`Utf8PathBuf`]."] # [inline] pub fn as_os_str (& self) -> & OsStr { & self . os_string } # [doc = " Returns the [`OsString`] that was attempted to be converted to [`Utf8PathBuf`]."] # [inline] pub fn into_os_string (self) -> OsString { self . os_string } # [doc = " Fetches a [`FromOsStrError`] for more about the conversion failure."] # [doc = ""] # [doc = " At the moment this struct does not contain any additional information, but is provided for"] # [doc = " completeness."] # [inline] pub fn from_os_str_error (& self) -> FromOsStrError { self . error } # [doc = " Converts self into a [`std::io::Error`] with kind"] # [doc = " [`InvalidData`](io::ErrorKind::InvalidData)."] # [doc = ""] # [doc = " Many users of [`FromOsStringError`] will want to convert it into an [`io::Error`]."] # [doc = " This is a convenience method to do that."] pub fn into_io_error (self) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , self) } }
};
}
