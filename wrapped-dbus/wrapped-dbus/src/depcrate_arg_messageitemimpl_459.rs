// Generated macro for impl_459 (impl)
macro_rules! Depcrate_arg_messageitemimpl_459 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_459"}
// Dependencies: {}
impl arg :: Append for MessageItemDict { fn append_by_ref (& self , i : & mut IterAppend) { i . append_container (ArgType :: Array , Some (self . element_signature ()) , | s | { for (k , v) in & self . v { s . append_container (ArgType :: DictEntry , None , | ss | { k . append_by_ref (ss) ; v . append_by_ref (ss) ; }) ; } }) ; } }
};
}
