// Generated macro for StringWriter (struct)
macro_rules! Depcrate_formatStringWriter {
() => {
// Module: crate::format
// Provides: {"StringWriter"}
// Dependencies: {}
# [doc = " Writer that fails (not panics) on OOM."] # [cfg (feature = "alloc")] struct StringWriter < 'a > { # [doc = " Destination buffer."] buffer : & 'a mut String , # [doc = " Memory allocation error."] error : Option < TryReserveError > , }
};
}
