// Generated macro for impl_322 (impl)
macro_rules! Depcrate_arg_basic_implimpl_322 {
() => {
// Module: crate::arg::basic_impl
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'a > Get < 'a > for & 'a str { fn get (i : & mut Iter < 'a >) -> Option < & 'a str > { unsafe { arg_get_str (& mut i . 0 , ArgType :: String) } . and_then (| s | s . to_str () . ok ()) } }
};
}
