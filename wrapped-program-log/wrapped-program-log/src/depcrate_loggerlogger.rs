// Generated macro for Logger (struct)
macro_rules! Depcrate_loggerLogger {
() => {
// Module: crate::logger
// Provides: {"Logger"}
// Dependencies: {}
# [doc = " Logger to efficiently format log messages."] # [doc = ""] # [doc = " The logger is a fixed size buffer that can be used to format log messages"] # [doc = " before sending them to the log output. Any type that implements the `Log`"] # [doc = " trait can be appended to the logger."] pub struct Logger < const BUFFER : usize > { buffer : [MaybeUninit < u8 > ; BUFFER] , len : usize , }
};
}
