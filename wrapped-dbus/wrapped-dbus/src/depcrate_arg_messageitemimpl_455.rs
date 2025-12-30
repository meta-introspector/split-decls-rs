// Generated macro for impl_455 (impl)
macro_rules! Depcrate_arg_messageitemimpl_455 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_455"}
// Dependencies: {}
impl arg :: Append for MessageItemArray { fn append_by_ref (& self , i : & mut IterAppend) { i . append_container (ArgType :: Array , Some (self . element_signature ()) , | s | { for a in & self . v { a . append_by_ref (s) } }) ; } }
};
}
