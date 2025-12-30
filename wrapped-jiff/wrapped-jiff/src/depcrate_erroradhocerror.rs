// Generated macro for AdhocError (struct)
macro_rules! Depcrate_errorAdhocError {
() => {
// Module: crate::error
// Provides: {"AdhocError"}
// Dependencies: {}
# [doc = " A generic error message."] # [doc = ""] # [doc = " This somewhat unfortunately represents most of the errors in Jiff. When I"] # [doc = " first started building Jiff, I had a goal of making every error structured."] # [doc = " But this ended up being a ton of work, and I find it much easier and nicer"] # [doc = " for error messages to be embedded where they occur."] # [cfg_attr (not (feature = "alloc") , derive (Clone))] struct AdhocError { # [cfg (feature = "alloc")] message : alloc :: boxed :: Box < str > , # [cfg (not (feature = "alloc"))] message : & 'static str , }
};
}
