// Generated macro for impl_325 (impl)
macro_rules! Depcrate_arg_basic_implimpl_325 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_325"}
// Dependencies: {}
impl < 'a > Append for String { fn append (mut self , i : & mut IterAppend) { self . push_str ("\0") ; let s : & str = & self ; s . append (i) } fn append_by_ref (& self , i : & mut IterAppend) { (& * * self) . append_by_ref (i) } }
};
}
