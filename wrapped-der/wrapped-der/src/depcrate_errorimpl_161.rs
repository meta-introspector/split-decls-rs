// Generated macro for impl_161 (impl)
macro_rules! Depcrate_errorimpl_161 {
() => {
// Module: crate::error
// Provides: {"impl_161"}
// Dependencies: {}
impl ErrorKind { # [doc = " Annotate an [`ErrorKind`] with context about where it occurred,"] # [doc = " returning an error."] pub fn at (self , position : Length) -> Error { Error :: new (self , position) } # [doc = " Convert to an error, omitting position information."] pub fn to_error (self) -> Error { Error :: from_kind (self) } }
};
}
