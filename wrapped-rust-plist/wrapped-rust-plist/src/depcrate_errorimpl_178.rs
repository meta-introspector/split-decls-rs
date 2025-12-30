// Generated macro for impl_178 (impl)
macro_rules! Depcrate_errorimpl_178 {
() => {
// Module: crate::error
// Provides: {"impl_178"}
// Dependencies: {}
impl Error { # [doc = " Returns true if this error was caused by a failure to read or write bytes on an IO stream."] pub fn is_io (& self) -> bool { self . as_io () . is_some () } # [doc = " Returns true if this error was caused by prematurely reaching the end of the input data."] pub fn is_eof (& self) -> bool { matches ! (self . inner . kind , ErrorKind :: UnexpectedEof) } # [doc = " Returns the underlying error if it was caused by a failure to read or write bytes on an IO"] # [doc = " stream."] pub fn as_io (& self) -> Option < & io :: Error > { if let ErrorKind :: Io (err) = & self . inner . kind { Some (err) } else { None } } # [doc = " Returns the underlying error if it was caused by a failure to read or write bytes on an IO"] # [doc = " stream or `self` if it was not."] pub fn into_io (self) -> Result < io :: Error , Self > { if let ErrorKind :: Io (err) = self . inner . kind { Ok (err) } else { Err (self) } } }
};
}
