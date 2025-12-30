// Generated macro for Rewind (struct)
macro_rules! Depcrate_common_io_rewindRewind {
() => {
// Module: crate::common::io::rewind
// Provides: {"Rewind"}
// Dependencies: {}
# [doc = " Combine a buffer with an IO, rewinding reads to use the buffer."] # [derive (Debug)] pub (crate) struct Rewind < T > { pre : Option < Bytes > , inner : T , }
};
}
