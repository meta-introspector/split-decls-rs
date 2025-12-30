// Generated macro for impl_475 (impl)
macro_rules! Depcrate_proto_streams_storeimpl_475 {
() => {
// Module: crate::proto::streams::store
// Provides: {"impl_475"}
// Dependencies: {}
# [cfg (feature = "unstable")] impl Drop for Store { fn drop (& mut self) { use std :: thread ; if ! thread :: panicking () { debug_assert ! (self . slab . is_empty ()) ; } } }
};
}
