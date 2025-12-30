// Generated macro for State (enum)
macro_rules! Depcrate_h3_streamState {
() => {
// Module: crate::h3::stream
// Provides: {"State"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum State { # [doc = " Reading the stream's type."] StreamType , # [doc = " Reading the stream's current frame's type."] FrameType , # [doc = " Reading the stream's current frame's payload length."] FramePayloadLen , # [doc = " Reading the stream's current frame's payload."] FramePayload , # [doc = " Reading DATA payload."] Data , # [doc = " Reading the push ID."] PushId , # [doc = " Reading a QPACK instruction."] QpackInstruction , # [doc = " Reading and discarding data."] Drain , # [doc = " All data has been read."] Finished , }
};
}
