// Generated macro for Error (struct)
macro_rules! Depcrate_shared_util_errorError {
() => {
// Module: crate::shared::util::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error that can be returned when parsing."] # [derive (Clone , Debug)] pub struct Error { # [cfg (feature = "alloc")] message : alloc :: boxed :: Box < str > , # [cfg (not (feature = "alloc"))] message : & 'static str , }
};
}
