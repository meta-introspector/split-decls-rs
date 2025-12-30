// Generated macro for from_io (function)
macro_rules! Depcrate_errorfrom_io {
() => {
// Module: crate::error
// Provides: {"from_io"}
// Dependencies: {}
pub fn from_io (e : io :: Error) -> Error { Error { kind : ErrorKind :: Io (e) , } }
};
}
