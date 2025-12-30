// Generated macro for from_io_without_position (function)
macro_rules! Depcrate_errorfrom_io_without_position {
() => {
// Module: crate::error
// Provides: {"from_io_without_position"}
// Dependencies: {}
pub (crate) fn from_io_without_position (err : io :: Error) -> Error { ErrorKind :: Io (err) . without_position () }
};
}
