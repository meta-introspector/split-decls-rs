// Generated macro for current_cbor_tag (function)
macro_rules! Depcrate_tagscurrent_cbor_tag {
() => {
// Module: crate::tags
// Provides: {"current_cbor_tag"}
// Dependencies: {}
# [doc = " function to get the current cbor tag"] # [doc = ""] # [doc = " The only place where it makes sense to call this function is within visit_newtype_struct of a serde visitor."] # [doc = " This is a low level API. In most cases it is preferable to use Tagged"] pub fn current_cbor_tag () -> Option < u64 > { get_tag () }
};
}
