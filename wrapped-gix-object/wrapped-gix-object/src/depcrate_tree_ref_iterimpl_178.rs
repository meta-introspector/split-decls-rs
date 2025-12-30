// Generated macro for impl_178 (impl)
macro_rules! Depcrate_tree_ref_iterimpl_178 {
() => {
// Module: crate::tree::ref_iter
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for tree :: EntryMode { type Error = & 'a [u8] ; fn try_from (mode : & 'a [u8]) -> Result < Self , Self :: Error > { tree :: EntryMode :: from_bytes (mode) . ok_or (mode) } }
};
}
