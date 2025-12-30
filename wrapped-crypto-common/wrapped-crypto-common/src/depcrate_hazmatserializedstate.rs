// Generated macro for SerializedState (type)
macro_rules! Depcrate_hazmatSerializedState {
() => {
// Module: crate::hazmat
// Provides: {"SerializedState"}
// Dependencies: {}
# [doc = " Serialized internal state."] pub type SerializedState < T > = Array < u8 , < T as SerializableState > :: SerializedStateSize > ;
};
}
