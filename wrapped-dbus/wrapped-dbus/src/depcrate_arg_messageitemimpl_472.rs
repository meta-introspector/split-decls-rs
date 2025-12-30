// Generated macro for impl_472 (impl)
macro_rules! Depcrate_arg_messageitemimpl_472 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_472"}
// Dependencies: {}
# [doc = " Create a `MessageItem::Array`."] impl < 'a , T > From < & 'a [T] > for MessageItem where T : Into < MessageItem > + Clone + Default { fn from (i : & 'a [T]) -> MessageItem { MessageItem :: new_array2 (i . iter () . cloned ()) } }
};
}
