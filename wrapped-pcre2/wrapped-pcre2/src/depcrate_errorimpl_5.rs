// Generated macro for impl_5 (impl)
macro_rules! Depcrate_errorimpl_5 {
() => {
// Module: crate::error
// Provides: {"impl_5"}
// Dependencies: {}
impl Error { pub (crate) fn regex < E : std :: error :: Error > (err : E) -> Error { Error { kind : ErrorKind :: Regex (err . to_string ()) } } # [doc = " Return the kind of this error."] pub fn kind (& self) -> & ErrorKind { & self . kind } }
};
}
