// Generated macro for impl_16 (impl)
macro_rules! Depcrate_errorimpl_16 {
() => {
// Module: crate::error
// Provides: {"impl_16"}
// Dependencies: {}
impl From < StripPrefixError > for Error { fn from (err : StripPrefixError) -> Error { Error :: new (ErrorKind :: StripPrefix (err) , "StripPrefixError. Look inside for more details" ,) } }
};
}
