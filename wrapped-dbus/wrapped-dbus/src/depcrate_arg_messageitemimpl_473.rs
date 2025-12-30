// Generated macro for impl_473 (impl)
macro_rules! Depcrate_arg_messageitemimpl_473 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_473"}
// Dependencies: {}
# [doc = " Create a `MessageItem::Dict`."] impl < 'a , T1 , T2 > From < & 'a [(T1 , T2)] > for MessageItem where T1 : Into < MessageItem > + Clone + Default , T2 : Into < MessageItem > + Clone + Default { fn from (i : & 'a [(T1 , T2)]) -> MessageItem { MessageItem :: new_dict2 (i . iter () . cloned ()) } }
};
}
