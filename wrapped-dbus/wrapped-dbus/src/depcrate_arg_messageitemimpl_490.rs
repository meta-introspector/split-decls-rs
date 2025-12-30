// Generated macro for impl_490 (impl)
macro_rules! Depcrate_arg_messageitemimpl_490 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_490"}
// Dependencies: {}
impl arg :: Append for MessageItem { fn append_by_ref (& self , i : & mut IterAppend) { match self { MessageItem :: Str (a) => a . append_by_ref (i) , MessageItem :: Bool (a) => a . append_by_ref (i) , MessageItem :: Byte (a) => a . append_by_ref (i) , MessageItem :: Int16 (a) => a . append_by_ref (i) , MessageItem :: Int32 (a) => a . append_by_ref (i) , MessageItem :: Int64 (a) => a . append_by_ref (i) , MessageItem :: UInt16 (a) => a . append_by_ref (i) , MessageItem :: UInt32 (a) => a . append_by_ref (i) , MessageItem :: UInt64 (a) => a . append_by_ref (i) , MessageItem :: Double (a) => a . append_by_ref (i) , MessageItem :: Array (a) => a . append_by_ref (i) , MessageItem :: Struct (a) => i . append_container (ArgType :: Struct , None , | s | { for v in a { v . append_by_ref (s) ; } }) , MessageItem :: Variant (a) => { i . append_container (ArgType :: Variant , Some (a . signature () . as_cstr ()) , | s | a . append_by_ref (s)) } , MessageItem :: Dict (a) => a . append_by_ref (i) , MessageItem :: ObjectPath (a) => a . append_by_ref (i) , MessageItem :: Signature (a) => a . append_by_ref (i) , # [cfg (not (feature = "stdfd"))] MessageItem :: UnixFd (a) => a . append_by_ref (i) , # [cfg (feature = "stdfd")] MessageItem :: UnixFd (a) => a . 0 . append_by_ref (i) , } } }
};
}
