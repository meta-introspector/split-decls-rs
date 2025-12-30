// Generated macro for from_io_offset_0 (function)
macro_rules! Depcrate_streamfrom_io_offset_0 {
() => {
// Module: crate::stream
// Provides: {"from_io_offset_0"}
// Dependencies: {}
fn from_io_offset_0 (err : io :: Error) -> Error { ErrorKind :: Io (err) . with_byte_offset (0) }
};
}
