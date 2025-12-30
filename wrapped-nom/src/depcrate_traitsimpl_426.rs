// Generated macro for impl_426 (impl)
macro_rules! Depcrate_traitsimpl_426 {
() => {
// Module: crate::traits
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'a , 'b > FindSubstring < & 'b str > for & 'a [u8] { fn find_substring (& self , substr : & 'b str) -> Option < usize > { self . find_substring (AsBytes :: as_bytes (substr)) } }
};
}
