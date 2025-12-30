// Generated macro for ErrorInner (struct)
macro_rules! Depcrate_errorErrorInner {
() => {
// Module: crate::error
// Provides: {"ErrorInner"}
// Dependencies: {}
# [derive (Debug)] # [cfg_attr (not (feature = "alloc") , derive (Clone))] struct ErrorInner { kind : ErrorKind , # [cfg (feature = "alloc")] cause : Option < Error > , }
};
}
