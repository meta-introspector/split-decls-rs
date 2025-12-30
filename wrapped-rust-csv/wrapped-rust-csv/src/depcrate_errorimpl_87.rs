// Generated macro for impl_87 (impl)
macro_rules! Depcrate_errorimpl_87 {
() => {
// Module: crate::error
// Provides: {"impl_87"}
// Dependencies: {}
impl Error { # [doc = " A crate private constructor for `Error`."] pub (crate) fn new (kind : ErrorKind) -> Error { Error (Box :: new (kind)) } # [doc = " Return the specific type of this error."] pub fn kind (& self) -> & ErrorKind { & self . 0 } # [doc = " Unwrap this error into its underlying type."] pub fn into_kind (self) -> ErrorKind { * self . 0 } # [doc = " Returns true if this is an I/O error."] # [doc = ""] # [doc = " If this is true, the underlying `ErrorKind` is guaranteed to be"] # [doc = " `ErrorKind::Io`."] pub fn is_io_error (& self) -> bool { matches ! (* self . 0 , ErrorKind :: Io (_)) } # [doc = " Return the position for this error, if one exists."] # [doc = ""] # [doc = " This is a convenience function that permits callers to easily access"] # [doc = " the position on an error without doing case analysis on `ErrorKind`."] pub fn position (& self) -> Option < & Position > { self . 0 . position () } }
};
}
