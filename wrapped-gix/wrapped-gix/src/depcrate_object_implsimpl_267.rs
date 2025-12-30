// Generated macro for impl_267 (impl)
macro_rules! Depcrate_object_implsimpl_267 {
() => {
// Module: crate::object::impls
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'repo > From < Commit < 'repo > > for ObjectDetached { fn from (mut v : Commit < 'repo >) -> Self { ObjectDetached { id : v . id , kind : gix_object :: Kind :: Commit , data : steal_from_freelist (& mut v . data) , } } }
};
}
