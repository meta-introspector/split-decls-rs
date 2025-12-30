// Generated macro for error_out_of_memory (function)
macro_rules! Depcrateerror_out_of_memory {
() => {
// Module: crate
// Provides: {"error_out_of_memory"}
// Dependencies: {}
# [cfg (not (feature = "std"))] # [inline (always)] fn error_out_of_memory (msg : & 'static str) -> Error { Error :: OutOfMemory (msg) }
};
}
