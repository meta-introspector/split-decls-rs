// Generated macro for error_unsupported (function)
macro_rules! Depcrateerror_unsupported {
() => {
// Module: crate
// Provides: {"error_unsupported"}
// Dependencies: {}
# [cfg (not (feature = "std"))] # [inline (always)] fn error_unsupported (msg : & 'static str) -> Error { Error :: Unsupported (msg) }
};
}
