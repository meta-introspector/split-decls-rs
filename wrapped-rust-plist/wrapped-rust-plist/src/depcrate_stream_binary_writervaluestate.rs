// Generated macro for ValueState (enum)
macro_rules! Depcrate_stream_binary_writerValueState {
() => {
// Module: crate::stream::binary_writer
// Provides: {"ValueState"}
// Dependencies: {}
enum ValueState { # [doc = " The value has not been assigned an object reference."] Unassigned , # [doc = " The value has been assigned an object reference but has not yet been written."] Unwritten (ObjectRef) , # [doc = " The value has been written with the given object reference."] Written (ObjectRef) , }
};
}
