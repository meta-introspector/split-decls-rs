// Generated macro for impl_49 (impl)
macro_rules! Depcrate_compareimpl_49 {
() => {
// Module: crate::compare
// Provides: {"impl_49"}
// Dependencies: {}
impl PartialEq < & str > for WildStr < '_ > { fn eq (& self , other : & & str) -> bool { if self . has_meta { meta_cmp (self . line , other) } else { self . line == * other } } }
};
}
