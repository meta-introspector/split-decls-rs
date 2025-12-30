// Generated macro for impl_270 (impl)
macro_rules! Depcrate_object_implsimpl_270 {
() => {
// Module: crate::object::impls
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'repo > From < Tree < 'repo > > for ObjectDetached { fn from (mut v : Tree < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Tree , data : steal_from_freelist (& mut v . data) , } } }
};
}
