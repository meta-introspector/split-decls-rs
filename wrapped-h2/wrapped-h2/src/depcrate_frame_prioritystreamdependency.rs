// Generated macro for StreamDependency (struct)
macro_rules! Depcrate_frame_priorityStreamDependency {
() => {
// Module: crate::frame::priority
// Provides: {"StreamDependency"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] pub struct StreamDependency { # [doc = " The ID of the stream dependency target"] dependency_id : StreamId , # [doc = " The weight for the stream. The value exposed (and set) here is always in"] # [doc = " the range [0, 255], instead of [1, 256] (as defined in section 5.3.2.)"] # [doc = " so that the value fits into a `u8`."] weight : u8 , # [doc = " True if the stream dependency is exclusive."] is_exclusive : bool , }
};
}
