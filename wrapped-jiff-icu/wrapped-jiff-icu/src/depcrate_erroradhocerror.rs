// Generated macro for AdhocError (struct)
macro_rules! Depcrate_errorAdhocError {
() => {
// Module: crate::error
// Provides: {"AdhocError"}
// Dependencies: {}
# [doc = " A generic error message."] # [derive (Clone , Debug)] struct AdhocError { # [cfg (feature = "alloc")] message : alloc :: boxed :: Box < str > , # [cfg (not (feature = "alloc"))] message : & 'static str , }
};
}
