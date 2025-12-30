// Generated macro for from_utf8 (function)
macro_rules! Depcrate_errorfrom_utf8 {
() => {
// Module: crate::error
// Provides: {"from_utf8"}
// Dependencies: {}
pub fn from_utf8 (e : str :: Utf8Error) -> Error { Error { kind : ErrorKind :: Utf8 (e) , } }
};
}
