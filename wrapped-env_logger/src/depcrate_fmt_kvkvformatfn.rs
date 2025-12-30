// Generated macro for KvFormatFn (type)
macro_rules! Depcrate_fmt_kvKvFormatFn {
() => {
// Module: crate::fmt::kv
// Provides: {"KvFormatFn"}
// Dependencies: {}
# [doc = " Format function for serializing key/value pairs"] # [doc = ""] # [doc = " This function determines how key/value pairs for structured logs are serialized within the default"] # [doc = " format."] pub (crate) type KvFormatFn = dyn Fn (& mut Formatter , & dyn Source) -> io :: Result < () > + Sync + Send ;
};
}
