// Generated macro for impl_269 (impl)
macro_rules! Depcrate_object_implsimpl_269 {
() => {
// Module: crate::object::impls
// Provides: {"impl_269"}
// Dependencies: {}
impl < 'repo > From < Blob < 'repo > > for ObjectDetached { fn from (mut v : Blob < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Blob , data : steal_from_freelist (& mut v . data) , } } }
};
}
