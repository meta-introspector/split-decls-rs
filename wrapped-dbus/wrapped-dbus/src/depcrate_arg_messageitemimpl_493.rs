// Generated macro for impl_493 (impl)
macro_rules! Depcrate_arg_messageitemimpl_493 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_493"}
// Dependencies: {}
impl arg :: Append for arg :: Variant < MessageItem > { fn append_by_ref (& self , i : & mut IterAppend) { let z = & self . 0 ; let asig = z . signature () ; let sig = asig . as_cstr () ; i . append_container (ArgType :: Variant , Some (& sig) , | s | z . append_by_ref (s)) ; } }
};
}
