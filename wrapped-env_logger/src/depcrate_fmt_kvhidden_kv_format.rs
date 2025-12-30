// Generated macro for hidden_kv_format (function)
macro_rules! Depcrate_fmt_kvhidden_kv_format {
() => {
// Module: crate::fmt::kv
// Provides: {"hidden_kv_format"}
// Dependencies: {}
# [doc = " Null Key Value Format"] # [doc = ""] # [doc = " This function is intended to be passed to"] # [doc = " [`Builder::format_key_values`](crate::Builder::format_key_values)."] # [doc = ""] # [doc = " This key value format simply ignores any key/value fields and doesn't include them in the"] # [doc = " output."] pub fn hidden_kv_format (_formatter : & mut Formatter , _fields : & dyn Source) -> io :: Result < () > { Ok (()) }
};
}
