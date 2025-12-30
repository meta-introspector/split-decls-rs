// Generated macro for RecvBufResetReturn (struct)
macro_rules! Depcrate_streamRecvBufResetReturn {
() => {
// Module: crate::stream
// Provides: {"RecvBufResetReturn"}
// Dependencies: {}
# [doc = " Return value type of `RecvBuf::reset()`"] # [derive (Debug , PartialEq , Clone , Copy)] pub struct RecvBufResetReturn { # [doc = " Returns the difference between the previous max_data offset"] # [doc = " received and the final size reported by the reset"] pub max_data_delta : u64 , # [doc = " The amount of flow control credit that should be returned to the"] # [doc = " connection level flow control."] pub consumed_flowcontrol : u64 , }
};
}
