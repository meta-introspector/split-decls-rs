// Generated macro for Rewind (struct)
macro_rules! Depcrate_common_rewindRewind {
() => {
// Module: crate::common::rewind
// Provides: {"Rewind"}
// Dependencies: {}
# [doc = " Combine a buffer with an IO, rewinding reads to use the buffer."] # [derive (Debug)] pub (crate) struct Rewind < T > { pub (crate) pre : Option < Bytes > , pub (crate) inner : T , }
};
}
