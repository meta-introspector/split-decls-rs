// Generated macro for SerializedBufferSize (type)
macro_rules! DepcrateSerializedBufferSize {
() => {
// Module: crate
// Provides: {"SerializedBufferSize"}
// Dependencies: {}
# [doc = " Size of serialized `BlockBuffer` in bytes."] pub type SerializedBufferSize < BS , K > = Sum < BS , < K as sealed :: Sealed > :: Overhead > ;
};
}
